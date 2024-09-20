use super::{routine::Routine, routine_registry::RoutineRegistry};

type GlobalInput = Vec<u8>;
type GlobalOutput = Vec<u8>;

static mut ROUTINE_REGISTRY: Option<RoutineRegistry<GlobalInput, GlobalOutput>> = None;

pub fn register_routine(routine: Routine<GlobalInput, GlobalOutput>) {
    unsafe {
        if ROUTINE_REGISTRY.is_none() {
            ROUTINE_REGISTRY = Some(RoutineRegistry::new());
        }
        ROUTINE_REGISTRY.as_mut().unwrap().register_routine(routine);
    }
}

pub fn get_routine_registry() -> &'static RoutineRegistry<GlobalInput, GlobalOutput> {
    unsafe { ROUTINE_REGISTRY.as_ref().unwrap() }
}

#[cfg(test)]
mod test {
    use super::*;

    #[global_routine]
    fn add(args: Vec<u8>) -> Vec<u8> {
        args.iter().map(|x| x + 1).collect()
    }

    #[test]
    fn should_return_routine() {
        let routine_name = Routine::new(add).name().to_owned();

        let registry = get_routine_registry();
        let result = registry.get_routine(routine_name.as_str());

        assert!(result.is_some());
    }
}
