// Test script to verify FFI functions work correctly
use std::ffi::CString;
use crate::*;

#[cfg(test)]
mod ffi_tests {
    use super::*;

    #[test]
    fn test_ffi_new_game() {
        let result_ptr = flip7_new_game(3, 42);
        assert!(!result_ptr.is_null());

        let result_str = unsafe {
            let cstr = std::ffi::CStr::from_ptr(result_ptr);
            cstr.to_string_lossy().into_owned()
        };

        println!("New game result: {}", result_str);

        // Parse JSON to verify structure
        let result: serde_json::Value = serde_json::from_str(&result_str).unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["players"], 3);
        assert_eq!(result["seed"], 42);

        flip7_free_string(result_ptr);
    }

    #[test]
    fn test_ffi_full_game_flow() {
        // Create a new game
        let new_game_result = flip7_new_game(2, 123);
        let result_str = unsafe {
            std::ffi::CStr::from_ptr(new_game_result).to_string_lossy().into_owned()
        };
        let result: serde_json::Value = serde_json::from_str(&result_str).unwrap();
        let game_id = result["game_id"].as_str().unwrap();
        flip7_free_string(new_game_result);

        // Get initial state
        let game_id_cstr = CString::new(game_id).unwrap();
        let state_result = flip7_get_state(game_id_cstr.as_ptr());
        let state_str = unsafe {
            std::ffi::CStr::from_ptr(state_result).to_string_lossy().into_owned()
        };
        println!("Initial state: {}", state_str);
        flip7_free_string(state_result);

        // Player 0 draws
        let draw_result = flip7_draw(game_id_cstr.as_ptr(), 0);
        let draw_str = unsafe {
            std::ffi::CStr::from_ptr(draw_result).to_string_lossy().into_owned()
        };
        println!("Draw result: {}", draw_str);
        flip7_free_string(draw_result);

        // Player 1 stays
        let stay_result = flip7_stay(game_id_cstr.as_ptr(), 1);
        let stay_str = unsafe {
            std::ffi::CStr::from_ptr(stay_result).to_string_lossy().into_owned()
        };
        println!("Stay result: {}", stay_str);
        flip7_free_string(stay_result);

        // Player 0 stays to finish round
        let stay_result = flip7_stay(game_id_cstr.as_ptr(), 0);
        let stay_str = unsafe {
            std::ffi::CStr::from_ptr(stay_result).to_string_lossy().into_owned()
        };
        println!("Final stay result: {}", stay_str);

        let stay_data: serde_json::Value = serde_json::from_str(&stay_str).unwrap();
        assert_eq!(stay_data["success"], true);
        assert_eq!(stay_data["round_finished"], true);

        flip7_free_string(stay_result);
    }
}