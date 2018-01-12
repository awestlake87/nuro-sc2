
use std::collections::HashMap;

use organelle;
use organelle::{ Cell, Handle, Protocol, ResultExt, Constraint };
use url::Url;
use uuid::Uuid;

use super::{
    Result,

    Message,
    Role,
    Organelle,
    Soma,

    GameSettings,
    GamePorts,
    PortSet,
    PlayerSetup,
};

use launcher::{ LauncherCell, LauncherSettings };

/// suite of games to choose from when pitting bots against each other
pub enum MeleeSuite {
    /// play one game with the given settings
    OneAndDone(GameSettings),
    /// repeat this game indefinitely
    EndlessRepeat(GameSettings),
}

/// settings for the melee cell
pub struct MeleeSettings<L1: Cell + 'static, L2: Cell + 'static> {
    /// the settings for the launcher cell
    pub launcher:   LauncherSettings,
    /// the player organelles
    pub players:    (L1, L2),
    /// the suite of games to choose from
    pub suite:      MeleeSuite,
}

/// cell designed to pit two bots against each other in Sc2 games
pub enum MeleeCell {
    /// wait for soma to gather effector, inputs, and outputs
    Init(Init),

    /// fetch player info in order to decide how many instances it needs
    Setup(Setup),
    /// gather instances and game ports, then transition to PVP or PVC
    Launch(Launch),

    /// coordinate two instances for player vs player
    PlayerVsPlayer(PlayerVsPlayer),
    /// coordinate one instance for player vs the built-in Sc2 AI
    PlayerVsComputer(PlayerVsComputer),

    /// melee suite is exhausted and organelle is awaiting shutdown
    Completed(Completed),
}

impl MeleeCell {
    /// melee cell only works as a controller in a melee organelle
    fn new(suite: MeleeSuite) -> Result<Self> {
        Ok(
            MeleeCell::Init(
                Init {
                    soma: Soma::new(
                        vec![ ],
                        vec![
                            Constraint::RequireOne(Role::Launcher),

                            Constraint::Variadic(Role::Controller),
                            Constraint::Variadic(Role::InstanceProvider),
                        ]
                    )?,

                    suite: suite,
                }
            )
        )
    }

    /// create the melee organelle
    pub fn organelle<L1, L2>(settings: MeleeSettings<L1, L2>) -> Result<Organelle>
        where
            L1: Cell,
            L2: Cell,

            Message: From<L1::Message> + From<L2::Message>,
            Role: From<L1::Role> + From<L2::Role>,

            L1::Message: From<Message>,
            L2::Message: From<Message>,

            L1::Role: From<Role>,
            L2::Role: From<Role>,
    {
        let mut organelle = Organelle::new(MeleeCell::new(settings.suite)?);

        let launcher = organelle.add_cell(LauncherCell::from(settings.launcher)?);

        let melee = organelle.get_main_handle();

        let player1 = organelle.add_cell(settings.players.0);
        let player2 = organelle.add_cell(settings.players.1);

        organelle.connect(melee, launcher, Role::Launcher);

        organelle.connect(melee, player1, Role::Controller);
        organelle.connect(melee, player2, Role::Controller);
        organelle.connect(melee, player1, Role::InstanceProvider);
        organelle.connect(melee, player2, Role::InstanceProvider);

        Ok(organelle)
    }
}

impl Cell for MeleeCell {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Self::Message, Self::Role>)
        -> organelle::Result<Self>
    {
        match self {
            MeleeCell::Init(state) => state.update(msg),
            MeleeCell::Setup(state) => state.update(msg),
            MeleeCell::Launch(state) => state.update(msg),
            MeleeCell::PlayerVsPlayer(state) => state.update(msg),
            MeleeCell::PlayerVsComputer(state) => state.update(msg),
            MeleeCell::Completed(state) => state.update(msg),
        }.chain_err(
            || organelle::ErrorKind::CellError
        )
    }
}

pub struct Init {
    soma:               Soma,
    suite:              MeleeSuite,
}

impl Init {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<MeleeCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Start => Setup::setup(self.soma, self.suite),


                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(MeleeCell::Init(self))
        }
    }
}

pub struct Setup {
    soma:               Soma,
    suite:              Option<MeleeSuite>,

    agents:             (Handle, Handle),
    clients:            (Handle, Handle),

    game:               GameSettings,
    players:            (Option<PlayerSetup>, Option<PlayerSetup>),
}

impl Setup {
    fn setup(soma: Soma, suite: MeleeSuite) -> Result<MeleeCell> {
        let clients = soma.var_output(Role::InstanceProvider)?.clone();
        let agents = soma.var_output(Role::Controller)?.clone();

        if clients.len() != 2 {
            bail!("expected 2 clients, got {}", clients.len())
        }

        if agents.len() != 2 {
            bail!("expected 2 agents, got {}", agents.len())
        }

        let (game, suite) = match suite {
            MeleeSuite::OneAndDone(game) => (game, None),
            MeleeSuite::EndlessRepeat(game) => {
                let suite = Some(MeleeSuite::EndlessRepeat(game.clone()));

                (game, suite)
            }
        };

        soma.effector()?.send(
            agents[0], Message::RequestPlayerSetup(game.clone())
        );
        soma.effector()?.send(
            agents[1], Message::RequestPlayerSetup(game.clone())
        );

        Ok(
            MeleeCell::Setup(
                Setup {
                    soma: soma,
                    suite: suite,

                    agents: (agents[0], agents[1]),
                    clients: (clients[0], clients[1]),

                    game: game,
                    players: (None, None),
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<MeleeCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::PlayerSetup(setup)) => {
                    self.on_player_setup(src, setup)
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(MeleeCell::Setup(self))
        }
    }

    fn on_player_setup(mut self, src: Handle, setup: PlayerSetup)
        -> Result<MeleeCell>
    {
        if src == self.agents.0 {
            self.players.0 = Some(setup);
        }
        else if src == self.agents.1 {
            self.players.1 = Some(setup);
        }
        else {
            bail!("invalid source for player setup")
        }

        match self.players {
            (Some(setup1), Some(setup2)) => {
                Launch::launch(
                    self.soma,
                    self.suite,
                    self.agents,
                    self.clients,
                    (setup1, setup2),
                    self.game
                )
            },

            _ => Ok(MeleeCell::Setup(self))
        }
    }
}

pub struct Launch {
    soma:                   Soma,
    suite:                  Option<MeleeSuite>,
    launcher:               Handle,

    agents:                 (Handle, Handle),
    clients:                (Handle, Handle),

    game:                   GameSettings,
    players:                (PlayerSetup, PlayerSetup),
    instances:              HashMap<Uuid, (Url, PortSet)>,
    ports:                  Vec<GamePorts>,

    is_pvp:                 bool,
    instances_requested:    u32
}

impl Launch {
    fn launch(
        soma: Soma,
        suite: Option<MeleeSuite>,
        agents: (Handle, Handle),
        clients: (Handle, Handle),
        players: (PlayerSetup, PlayerSetup),
        game: GameSettings,
    )
        -> Result<MeleeCell>
    {
        let is_pvp = {
            if players.0.is_player() && players.1.is_computer() {
                false
            }
            else if players.0.is_computer() && players.1.is_player() {
                false
            }
            else if players.0.is_player() && players.1.is_player() {
                true
            }
            else {
                bail!("invalid player setups")
            }
        };

        let launcher = soma.req_output(Role::Launcher)?;
        soma.effector()?.send(launcher, Message::GetInstancePool);
        soma.effector()?.send(launcher, Message::GetPortsPool);

        Ok(
            MeleeCell::Launch(
                Launch {
                    soma: soma,
                    suite: suite,
                    launcher: launcher,

                    agents: agents,
                    clients: clients,

                    game: game,
                    players: players,
                    instances: HashMap::new(),
                    ports: vec![ ],

                    is_pvp: is_pvp,
                    instances_requested: 0,
                }
            )
        )
    }
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<MeleeCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::InstancePool(instances)) => {
                    self.on_instance_pool(src, instances)
                },
                Protocol::Message(src, Message::PortsPool(ports)) => {
                    self.on_ports_pool(src, ports)
                },


                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(MeleeCell::Launch(self))
        }
    }

    fn on_instance_pool(
        mut self, src: Handle, instances: HashMap<Uuid, (Url, PortSet)>
    )
        -> Result<MeleeCell>
    {
        assert_eq!(src, self.launcher);

        self.instances = instances;

        self.launch_instances()?;
        self.try_provide_instances()
    }

    fn on_ports_pool(mut self, src: Handle, ports: Vec<GamePorts>)
        -> Result<MeleeCell>
    {
        assert_eq!(src, self.launcher);

        self.ports = ports;

        self.launch_instances()?;
        self.try_provide_instances()
    }

    fn launch_instances(&mut self) -> Result<()> {
        if self.is_pvp {
            if self.instances.len() < 2 && self.instances_requested < 2 {
                // launch as many instances as needed
                while self.instances_requested < 2 {
                    self.soma.send_req_output(
                        Role::Launcher, Message::LaunchInstance
                    )?;

                    self.instances_requested += 1;
                }
            }
        }
        else {
            if self.instances.len() < 1 && self.instances_requested == 0 {
                self.soma.send_req_output(
                    Role::Launcher, Message::LaunchInstance
                )?;
                self.instances_requested = 1;
            }
        }

        Ok(())
    }

    fn try_provide_instances(self) -> Result<MeleeCell> {
        if self.is_pvp {
            if self.instances.len() >= 2 && self.ports.len() >= 1 {
                let (id1, &(ref url1, ref ports1)) = self.instances.iter()
                    .nth(0).unwrap()
                ;
                let (id2, &(ref url2, ref ports2)) = self.instances.iter()
                    .nth(1).unwrap()
                ;
                let mut ports = self.ports[0].clone();

                ports.client_ports = vec![ *ports1, *ports2 ];

                self.soma.effector()?.send(
                    self.clients.0,
                    Message::ProvideInstance(*id1, url1.clone())
                );
                self.soma.effector()?.send(
                    self.clients.1,
                    Message::ProvideInstance(*id2, url2.clone())
                );

                PlayerVsPlayer::start(
                    self.soma,
                    self.suite,
                    self.agents,
                    self.players,
                    self.game,
                    ports
                )
            }
            else {
                Ok(MeleeCell::Launch(self))
            }
        }
        else if self.instances.len() >= 1 {
            let (id, &(ref url, _)) = self.instances.iter()
                .nth(0).unwrap()
            ;

            let ((player, player_setup), (computer, computer_setup)) = {
                if self.players.0.is_player() {
                    (
                        (self.agents.0, self.players.0),
                        (self.agents.1, self.players.1),
                    )
                }
                else {
                    assert!(self.players.1.is_player());

                    (
                        (self.agents.1, self.players.1),
                        (self.agents.0, self.players.0),
                    )
                }
            };

            self.soma.effector()?.send(
                player, Message::ProvideInstance(*id, url.clone())
            );

            PlayerVsComputer::start(
                self.soma,
                self.suite,
                (player, player_setup),
                (computer, computer_setup),
                self.game,
            )
        }
        else {
            Ok(MeleeCell::Launch(self))
        }
    }
}

pub struct PlayerVsPlayer {
    soma: Soma,
    suite: Option<MeleeSuite>,

    agents: (Handle, Handle),

    game: GameSettings,
    ports: GamePorts,
    players: (PlayerSetup, PlayerSetup),

    ready: (bool, bool),
    ended: (bool, bool),
}

impl PlayerVsPlayer {
    fn start(
        soma: Soma,
        suite: Option<MeleeSuite>,
        agents: (Handle, Handle),
        players: (PlayerSetup, PlayerSetup),
        game: GameSettings,
        ports: GamePorts
    )
        -> Result<MeleeCell>
    {
        Ok(
            MeleeCell::PlayerVsPlayer(
                PlayerVsPlayer {
                    soma: soma,
                    suite: suite,

                    agents: agents,

                    game: game,
                    ports: ports,
                    players: players,

                    ready: (false, false),
                    ended: (false, false),
                }
            )
        )
    }
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<MeleeCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::Ready) => {
                    self.on_agent_ready(src)
                },
                Protocol::Message(src, Message::GameCreated) => {
                    self.on_game_created(src)
                },
                Protocol::Message(src, Message::GameEnded) => {
                    self.on_game_ended(src)
                },


                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(MeleeCell::PlayerVsPlayer(self))
        }
    }

    fn on_agent_ready(mut self, src: Handle) -> Result<MeleeCell> {
        if src == self.agents.0 {
            self.ready.0 = true;
        }
        else if src == self.agents.1 {
            self.ready.1 = true;
        }
        else {
            bail!("expected source of Ready to be an agent")
        }

        if self.ready == (true, true) {
            self.soma.effector()?.send(
                self.agents.0,
                Message::CreateGame(
                    self.game.clone(), vec![ self.players.0, self.players.1 ]
                )
            );
        }

        Ok(MeleeCell::PlayerVsPlayer(self))
    }

    fn on_game_created(self, src: Handle) -> Result<MeleeCell> {
        assert_eq!(src, self.agents.0);

        self.soma.effector()?.send(
            self.agents.0,
            Message::GameReady(self.players.0, Some(self.ports.clone()))
        );
        self.soma.effector()?.send(
            self.agents.1,
            Message::GameReady(self.players.1, Some(self.ports.clone()))
        );

        Ok(MeleeCell::PlayerVsPlayer(self))
    }

    fn on_game_ended(mut self, src: Handle) -> Result<MeleeCell> {
        if src == self.agents.0 {
            self.ended.0 = true;
        }
        else if src == self.agents.1 {
            self.ended.1 = true;
        }
        else {
            bail!("expected src of GameEnded to be an agent")
        }

        if self.ended == (true, true) {
            if self.suite.is_none() {
                Completed::complete(self.soma)
            }
            else {
                Setup::setup(self.soma, self.suite.unwrap())
            }
        }
        else {
            Ok(MeleeCell::PlayerVsPlayer(self))
        }
    }
}

/// MeleeCell state that pits players against the built-in AI
pub struct PlayerVsComputer {
    soma:               Soma,
    suite:              Option<MeleeSuite>,

    game:               GameSettings,
    player_setup:       PlayerSetup,
    computer_setup:     PlayerSetup,

    player:             Handle,
}

impl PlayerVsComputer {
    fn start(
        soma: Soma,
        suite: Option<MeleeSuite>,
        player: (Handle, PlayerSetup),
        computer: (Handle, PlayerSetup),
        game: GameSettings,
    )
        -> Result<MeleeCell>
    {
        Ok(
            MeleeCell::PlayerVsComputer(
                PlayerVsComputer {
                    soma: soma,
                    suite: suite,

                    game: game,
                    player_setup: player.1,
                    computer_setup: computer.1,

                    player: player.0,
                }
            )
        )
    }
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<MeleeCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::Ready) => {
                    self.on_agent_ready(src)
                },
                Protocol::Message(src, Message::GameCreated) => {
                    self.on_game_created(src)
                },
                Protocol::Message(src, Message::GameEnded) => {
                    self.on_game_ended(src)
                }


                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(MeleeCell::PlayerVsComputer(self))
        }
    }

    fn on_agent_ready(self, src: Handle) -> Result<MeleeCell> {
        if src != self.player {
            bail!("expected source of Ready to be the agent")
        }

        self.soma.effector()?.send(
            self.player,
            Message::CreateGame(
                self.game.clone(),
                vec![ self.player_setup, self.computer_setup ]
            )
        );

        Ok(MeleeCell::PlayerVsComputer(self))
    }

    fn on_game_created(self, src: Handle) -> Result<MeleeCell> {
        if src != self.player {
            bail!("expected source of GameCreated to be the agent")
        }

        self.soma.effector()?.send(
            self.player,
            Message::GameReady(self.player_setup, None)
        );

        Ok(MeleeCell::PlayerVsComputer(self))
    }

    fn on_game_ended(self, src: Handle) -> Result<MeleeCell> {
        if src != self.player {
            bail!("expected source of GameEnded to be an agent")
        }

        if self.suite.is_none() {
            Completed::complete(self.soma)
        }
        else {
            Setup::setup(self.soma, self.suite.unwrap())
        }
    }
}

pub struct Completed {
    soma:               Soma,
}

impl Completed {
    fn complete(soma: Soma) -> Result<MeleeCell> {
        soma.effector()?.stop();

        Ok(
            MeleeCell::Completed(Completed { soma: soma })
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<MeleeCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(MeleeCell::Completed(self))
        }
    }
}