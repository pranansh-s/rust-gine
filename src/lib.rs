pub mod graphics;
pub mod resource_manager;
pub mod entity;
pub mod character;
pub mod camera;
pub mod utility;
pub mod ui;

#[cfg(test)]
mod tests {
    use crate::utility::time::{TimeSystem, Timer};
    use crate::ui::ui_component::Div;
    use std::time::Duration;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_timer_one_shot() {
        let mut timer = Timer::new(Duration::from_millis(100), false);
        assert!(!timer.is_finished());
        
        let fired = timer.tick(Duration::from_millis(50));
        assert!(!fired);
        assert!(!timer.is_finished());
        
        let fired = timer.tick(Duration::from_millis(60));
        assert!(fired);
        assert!(timer.is_finished());
        
        let fired = timer.tick(Duration::from_millis(10));
        assert!(!fired);
    }

    #[test]
    fn test_timer_repeating() {
        let mut timer = Timer::new(Duration::from_millis(100), true);
        
        let fired = timer.tick(Duration::from_millis(110));
        assert!(fired);
        assert!(!timer.is_finished());
        
        let fired = timer.tick(Duration::from_millis(90));
        assert!(fired);
    }

    #[test]
    fn test_time_system_scheduling() {
        let mut time_system = TimeSystem::new(60.0);
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = executed.clone();
        
        time_system.schedule_once(Duration::from_millis(10), move || {
            executed_clone.store(true, Ordering::Relaxed);
        });
        
        time_system.update();
        std::thread::sleep(Duration::from_millis(15));
        time_system.update();
        
        assert!(executed.load(Ordering::Relaxed));
    }

    #[test]
    fn test_ui_hierarchy() {
        let mut root = Div::new(1, "root", vec![]);
        let child = Div::new(2, "child", vec![]);
        root.add_child(child);
        
        assert_eq!(root.id, 1);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].id, 2);
    }
}