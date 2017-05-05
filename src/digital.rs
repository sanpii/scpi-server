#[derive(Debug)]
pub enum Command {
    Reset,
    PinState,
    PinStateQuery,
    PinDirection,
    PinDirectionQuery,
    Unknow,
}

impl ::std::convert::From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "DIG:RST" => Command::Reset,
            "DIG:PIN" => Command::PinState,
            "DIG:PIN?" => Command::PinStateQuery,
            "DIG:PIN:DIR" => Command::PinDirection,
            "DIG:PIN:DIR?" => Command::PinDirectionQuery,
            _ => Command::Unknow,
        }
    }
}

pub fn execute(command: Command, args: Vec<String>) -> ::Result {
    match command {
        Command::Reset => reset(args),
        Command::PinState => set_pin_state(args),
        Command::PinStateQuery => get_pin_state(args),
        Command::PinDirection => set_pin_direction(args),
        Command::PinDirectionQuery => get_pin_direction(args),
        Command::Unknow => Err("Unknow command".to_owned()),
    }
}

fn reset(_: Vec<String>) -> ::Result {
    match ::redpitaya::pin::digital::reset() {
        Ok(_) => Ok(None),
        Err(err) => Err(err),
    }
}

fn set_pin_state(args: Vec<String>) -> ::Result {
    let pin = match args.get(0) {
        Some(pin) => pin.clone().into(),
        None => return Err("Missing parameter".to_owned()),
    };

    let state = match args.get(1) {
        Some(state) => state.parse::<u8>().unwrap().into(),
        None => return Err("Missing parameter".to_owned()),
    };

    match ::redpitaya::pin::digital::set_state(pin, state) {
        Ok(_) => Ok(None),
        Err(err) => Err(err),
    }
}

fn get_pin_state(args: Vec<String>) -> ::Result {
    let pin = match args.get(0) {
        Some(pin) => pin.clone().into(),
        None => return Err("Missing parameter".to_owned()),
    };

    match ::redpitaya::pin::digital::get_state(pin) {
        Ok(state) => Ok(Some(format!("{}", ::std::convert::Into::<u8>::into(state)))),
        Err(err) => Err(err),
    }
}

fn set_pin_direction(args: Vec<String>) -> ::Result {
    let pin = match args.get(0) {
        Some(pin) => pin.clone().into(),
        None => return Err("Missing parameter".to_owned()),
    };

    let direction = match args.get(1) {
        Some(direction) => direction.clone().into(),
        None => return Err("Missing parameter".to_owned()),
    };

    match ::redpitaya::pin::digital::set_direction(pin, direction) {
        Ok(_) => Ok(None),
        Err(err) => Err(err),
    }
}

fn get_pin_direction(args: Vec<String>) -> ::Result {
    let pin = match args.get(0) {
        Some(pin) => pin.clone().into(),
        None => return Err("Missing parameter".to_owned()),
    };

    match ::redpitaya::pin::digital::get_direction(pin) {
        Ok(direction) => Ok(Some(direction.into())),
        Err(err) => Err(err),
    }
}
