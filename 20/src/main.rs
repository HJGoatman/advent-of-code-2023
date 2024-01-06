use module_network::ModuleNetwork;
use std::env;
use std::fmt::Debug;
use std::fs;

trait Module: Debug {
    fn process(&mut self, from: &ModuleName, pulse: Pulse) -> Option<Pulse>;

    fn reset(&mut self);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ModuleName(String);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

mod module_network {
    use std::{collections::VecDeque, fmt::Debug, str::FromStr};

    use crate::{
        broadcast::Broadcast, conjunction::Conjunction, flip_flop::FlipFlop, Module, ModuleName,
        Pulse,
    };

    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum ParseModuleNetworkError {
        UnknownModuleType,
        FileFormatError,
    }

    type Modules = HashMap<ModuleName, Box<dyn Module>>;
    type Connections = HashMap<ModuleName, Vec<ModuleName>>;

    #[derive(Debug)]
    pub(crate) struct ModuleNetwork {
        modules: Modules,
        connections: Connections,
        total_low_pulses: u32,
        total_high_pulses: u32,
        total_button_pushes: u32,
    }

    const BROADCASTER_NAME: &str = "broadcaster";

    impl ModuleNetwork {
        fn new(modules: Modules, connections: Connections) -> ModuleNetwork {
            ModuleNetwork {
                modules,
                connections,
                total_low_pulses: 0,
                total_high_pulses: 0,
                total_button_pushes: 0,
            }
        }

        pub(crate) fn push_button(&mut self) {
            self.total_button_pushes += 1;

            const INITIAL_PULSE: Pulse = Pulse::Low;
            let initial_receiver: ModuleName = ModuleName(BROADCASTER_NAME.to_string());

            let initial_sender: ModuleName = ModuleName("button".to_string());

            let mut pulse_queue = VecDeque::new();
            pulse_queue.push_back((&initial_sender, &initial_receiver, INITIAL_PULSE));

            while let Some((sender, receiver, pulse)) = pulse_queue.pop_front() {
                match pulse {
                    Pulse::High => self.total_high_pulses += 1,
                    Pulse::Low => self.total_low_pulses += 1,
                };

                log::trace!("{:?} sends {:?} to {:?}", sender, pulse, receiver);
                if let Some(module) = self.modules.get_mut(receiver) {
                    if let Some(next_pulse) = module.process(sender, pulse) {
                        for target_box in ["ph", "nz", "tx", "dd"] {
                            if *receiver == ModuleName(target_box.to_string())
                                && next_pulse == Pulse::High
                            {
                                log::debug!(
                                    "inner_box: {} => {}",
                                    target_box,
                                    self.total_button_pushes
                                );
                            }
                        }

                        let next_receivers = self.connections.get(receiver).unwrap();

                        for next_receiver in next_receivers {
                            pulse_queue.push_back((receiver, next_receiver, next_pulse));
                        }
                    }
                }
            }
        }

        pub(crate) fn get_total_low_pulses_sent(&self) -> u32 {
            self.total_low_pulses
        }

        pub(crate) fn get_total_high_pulses_sent(&self) -> u32 {
            self.total_high_pulses
        }

        pub(crate) fn reset(&mut self) {
            self.modules
                .iter_mut()
                .for_each(|(_, module)| module.reset());

            self.total_low_pulses = 0;
            self.total_high_pulses = 0;
            self.total_button_pushes = 0;
        }

        pub(crate) fn get_total_button_pushes(&self) -> u32 {
            self.total_button_pushes
        }
    }

    type IntermediateParseResult<'a> = Vec<(ModuleName, &'a str, Vec<ModuleName>)>;

    impl FromStr for ModuleNetwork {
        type Err = ParseModuleNetworkError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let intermediate_parse_result: Result<
                IntermediateParseResult,
                ParseModuleNetworkError,
            > = s
                .split('\n')
                .map(|line| {
                    let [module_str, connections_str] = line
                        .split(" -> ")
                        .collect::<Vec<&str>>()
                        .try_into()
                        .map_err(|_| ParseModuleNetworkError::FileFormatError)?;

                    let connections: Vec<ModuleName> = connections_str
                        .split(", ")
                        .map(|connection_str| ModuleName(connection_str.to_string()))
                        .collect();

                    let module_name;
                    let module_type_str;
                    if module_str == BROADCASTER_NAME {
                        module_name = ModuleName(module_str.to_string());
                        module_type_str = module_str;
                    } else {
                        module_name = ModuleName(module_str[1..].to_string());
                        module_type_str = &module_str[0..1];
                    }

                    Ok((module_name, module_type_str, connections))
                })
                .collect();

            let intermediate_parse = intermediate_parse_result?;

            let modules = intermediate_parse
                .iter()
                .map(|(module_name, module_type_str, _)| -> Result<(ModuleName, Box<dyn Module>), ParseModuleNetworkError> {
                    let module: Result<Box<dyn Module>, ParseModuleNetworkError> =
                        match *module_type_str {
                            BROADCASTER_NAME => Ok(Box::new(Broadcast::new())),
                            "%" => Ok(Box::new(FlipFlop::new())),
                            "&" => {
                                let mut conjunction = Conjunction::new();
                                intermediate_parse
                                    .iter()
                                    .filter(|(_, _, connections)| {
                                        connections.contains(module_name)
                                    })
                                    .for_each(|(other_module, _, _)| {
                                        conjunction.connect(other_module.clone())
                                    });
                                Ok(Box::new(conjunction))
                            }
                            _ => Err(ParseModuleNetworkError::UnknownModuleType),
                        };

                    Ok((module_name.clone(), module?))
                })
                .collect::<Result<HashMap<ModuleName, Box<dyn Module>>, ParseModuleNetworkError>>()?;

            let connections = intermediate_parse
                .into_iter()
                .map(|(module_name, _, connections)| (module_name, connections))
                .collect();

            let module_network = ModuleNetwork::new(modules, connections);

            Ok(module_network)
        }
    }
}

mod flip_flop {
    use super::Module;
    use super::Pulse;
    use crate::ModuleName;

    #[derive(Debug)]
    pub(crate) enum FlipFlopState {
        On,
        Off,
    }

    #[derive(Debug)]
    pub(crate) struct FlipFlop {
        state: FlipFlopState,
    }

    const INITIAL_STATE: FlipFlopState = FlipFlopState::Off;

    impl FlipFlop {
        pub fn new() -> FlipFlop {
            FlipFlop {
                state: INITIAL_STATE,
            }
        }
    }

    impl Module for FlipFlop {
        fn process(&mut self, _from: &ModuleName, pulse: Pulse) -> Option<Pulse> {
            match pulse {
                Pulse::High => None,
                Pulse::Low => match self.state {
                    FlipFlopState::On => {
                        self.state = FlipFlopState::Off;
                        Some(Pulse::Low)
                    }
                    FlipFlopState::Off => {
                        self.state = FlipFlopState::On;
                        Some(Pulse::High)
                    }
                },
            }
        }

        fn reset(&mut self) {
            self.state = INITIAL_STATE;
        }
    }
}

mod conjunction {
    use crate::ModuleName;

    use super::Module;

    use super::Pulse;

    use std::collections::HashMap;

    #[derive(Debug)]
    pub(crate) struct Conjunction {
        memory: HashMap<ModuleName, Pulse>,
    }

    impl Conjunction {
        pub fn new() -> Conjunction {
            let memory = HashMap::new();

            Conjunction { memory }
        }

        const DEFAULT_PULSE: Pulse = Pulse::Low;

        pub fn connect(&mut self, module: ModuleName) {
            let _ = &mut self.memory.insert(module, Conjunction::DEFAULT_PULSE);
        }
    }

    impl Module for Conjunction {
        fn process(&mut self, from: &ModuleName, pulse: Pulse) -> Option<Pulse> {
            let last_pulse = self.memory.get_mut(from).unwrap();
            *last_pulse = pulse;

            if self.memory.values().all(|pulse| *pulse == Pulse::High) {
                Some(Pulse::Low)
            } else {
                Some(Pulse::High)
            }
        }

        fn reset(&mut self) {
            self.memory
                .iter_mut()
                .for_each(|(_, pulse)| *pulse = Conjunction::DEFAULT_PULSE);
        }
    }
}

mod broadcast {
    use crate::ModuleName;

    use super::Pulse;

    use super::Module;

    #[derive(Debug)]
    pub(crate) struct Broadcast {}

    impl Broadcast {
        pub(crate) fn new() -> Broadcast {
            Broadcast {}
        }
    }

    impl Module for Broadcast {
        fn process(&mut self, _from: &ModuleName, pulse: Pulse) -> Option<Pulse> {
            Some(pulse)
        }

        fn reset(&mut self) {}
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let mut module_network: ModuleNetwork = input.parse().unwrap();
    log::debug!("{:#?}", module_network);

    for _ in 0..1000 {
        module_network.push_button();
        log::trace!("");
    }

    log::debug!("{:?}", module_network);

    let pulse_product =
        module_network.get_total_low_pulses_sent() * module_network.get_total_high_pulses_sent();

    println!("{}", pulse_product);

    module_network.reset();

    for _ in 0..20000 {
        module_network.push_button();
    }

    // log::debug!("{:#?}", module_network);

    let total_button_pushes = module_network.get_total_button_pushes();
    println!("{}", total_button_pushes);
}
