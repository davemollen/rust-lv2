extern crate lv2_sys as sys;
mod feature;
pub use feature::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlPortStateUpdateErr {
    /// Unknown error.
    Unknown,
    /// Failed due to invalid port index.
    InvalidIndex,
}

impl ControlPortStateUpdateErr {
    /// Convert a raw status flag to a result or possible error value.
    pub fn from(
        value: sys::LV2_Control_Port_State_Update_Status,
    ) -> Result<(), ControlPortStateUpdateErr> {
        match value {
            sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_SUCCESS => Ok(()),
            sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_ERR_INVALID_INDEX => Err(ControlPortStateUpdateErr::InvalidIndex),
            sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_ERR_UNKNOWN => Err(ControlPortStateUpdateErr::Unknown),
            _ => Err(ControlPortStateUpdateErr::Unknown),
        }
    }

    /// Convert a result to a raw status flag.
    pub fn into(
        result: Result<(), ControlPortStateUpdateErr>,
    ) -> sys::LV2_Control_Port_State_Update_Status {
        match result {
            Ok(()) => sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_SUCCESS,
            Err(ControlPortStateUpdateErr::InvalidIndex) => {
                sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_ERR_INVALID_INDEX
            }
            Err(ControlPortStateUpdateErr::Unknown) => sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_ERR_UNKNOWN,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlPortState {
    /// No special state / Remove any previously set states.
    None,
    /// Inactive state (updates to port value are inaudible / ineffective).
    Inactive,
    /// Blocked state (updates to port value are ignored by the plugin and they should be blocked and ignored by the host).
    Blocked,
}

impl ControlPortState {
    /// Convert a raw status flag to a ControlPortState enum.
    pub fn from(value: sys::LV2_Control_Port_State) -> ControlPortState {
        match value {
            sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_BLOCKED => ControlPortState::Blocked,
            sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_INACTIVE => {
                ControlPortState::Inactive
            }
            sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_NONE => ControlPortState::None,
            _ => ControlPortState::None,
        }
    }

    /// Convert a ControlPortState enum to a raw status flag.
    pub fn into(state: ControlPortState) -> sys::LV2_Control_Port_State {
        match state {
            ControlPortState::None => sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_NONE,
            ControlPortState::Inactive => {
                sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_INACTIVE
            }
            ControlPortState::Blocked => sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_BLOCKED,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ControlPortState, ControlPortStateUpdateErr};

    #[test]
    fn test_control_port_state_err_conversion() {
        assert_eq!(
            Ok(()),
            ControlPortStateUpdateErr::from(
                sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_SUCCESS
            )
        );
        assert_eq!(
            Err(ControlPortStateUpdateErr::InvalidIndex),
            ControlPortStateUpdateErr::from(sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_ERR_INVALID_INDEX),
        );
        assert_eq!(
            Err(ControlPortStateUpdateErr::Unknown),
            ControlPortStateUpdateErr::from(
                sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_ERR_UNKNOWN
            ),
        );

        assert_eq!(
            ControlPortStateUpdateErr::into(Ok(())),
            sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_SUCCESS
        );
        assert_eq!(
            ControlPortStateUpdateErr::into(Err(ControlPortStateUpdateErr::InvalidIndex)),
            sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_ERR_INVALID_INDEX
        );
        assert_eq!(
            ControlPortStateUpdateErr::into(Err(ControlPortStateUpdateErr::Unknown)),
            sys::LV2_Control_Port_State_Update_Status_LV2_CONTROL_PORT_STATE_UPDATE_ERR_UNKNOWN
        );
    }

    #[test]
    fn test_control_port_state_conversion() {
        assert_eq!(
            ControlPortState::from(sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_NONE),
            ControlPortState::None
        );
        assert_eq!(
            ControlPortState::from(sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_INACTIVE),
            ControlPortState::Inactive
        );
        assert_eq!(
            ControlPortState::from(sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_BLOCKED),
            ControlPortState::Blocked
        );

        assert_eq!(
            ControlPortState::into(ControlPortState::None),
            sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_NONE
        );
        assert_eq!(
            ControlPortState::into(ControlPortState::Inactive),
            sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_INACTIVE
        );
        assert_eq!(
            ControlPortState::into(ControlPortState::Blocked),
            sys::LV2_Control_Port_State_LV2_CONTROL_PORT_STATE_BLOCKED
        );
    }
}
