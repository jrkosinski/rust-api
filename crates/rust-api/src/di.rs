//! Dependency Injection Container
//!
//! A simple, type-safe DI container that stores services as Arc-wrapped trait objects.
//! Services can be registered and retrieved by type, with automatic Arc wrapping.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

/// Trait that all injectable services must implement
pub trait Injectable: Send + Sync + 'static {}

/// Type-erased service storage using Any
type ServiceBox = Arc<dyn Any + Send + Sync>;

/// Dependency injection container
///
/// Stores services as Arc-wrapped values and provides type-safe retrieval.
/// Services are singletons - only one instance exists per type.
///
/// # Example
///
/// ```ignore
/// let mut container = Container::new();
/// container.register(Arc::new(DatabaseService::new()));
///
/// let db: Arc<DatabaseService> = container.resolve().unwrap();
/// ```
#[derive(Clone, Default)]
pub struct Container {
    services: HashMap<TypeId, ServiceBox>,
}

impl Container {
    /// Create a new empty container
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Register a service in the container
    ///
    /// The service must be wrapped in an Arc. If a service of this type
    /// already exists, it will be replaced.
    ///
    /// # Example
    ///
    /// ```ignore
    /// container.register(Arc::new(MyService::new()));
    /// ```
    pub fn register<T: Injectable>(&mut self, service: Arc<T>) {
        let type_id = self.get_type_id::<T>();
        self.insert_service(type_id, service);
    }

    //get the TypeId for a given type T
    fn get_type_id<T: Injectable>(&self) -> TypeId {
        TypeId::of::<T>()
    }

    //insert a service into the storage map
    fn insert_service<T: Injectable>(&mut self, type_id: TypeId, service: Arc<T>) {
        self.services.insert(type_id, service as ServiceBox);
    }

    /// Register a service from a constructor function
    ///
    /// This is a convenience method that creates the Arc for you.
    ///
    /// # Example
    ///
    /// ```ignore
    /// container.register_factory(|| MyService::new());
    /// ```
    pub fn register_factory<T: Injectable, F>(&mut self, factory: F)
    where
        F: FnOnce() -> T,
    {
        let service = self.create_service(factory);
        self.register(service);
    }

    //create a service instance from a factory function
    fn create_service<T: Injectable, F>(& self, factory: F) -> Arc<T>
    where
        F: FnOnce() -> T,
    {
        Arc::new(factory())
    }

    /// Resolve a service from the container
    ///
    /// Returns None if the service hasn't been registered.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let service: Arc<MyService> = container.resolve().unwrap();
    /// ```
    pub fn resolve<T: Injectable>(&self) -> Option<Arc<T>> {
        let type_id = self.get_type_id::<T>();
        self.lookup_service(type_id)
    }

    //lookup a service by TypeId and downcast it
    fn lookup_service<T: Injectable>(&self, type_id: TypeId) -> Option<Arc<T>> {
        self.services
            .get(&type_id)
            .and_then(|boxed| self.downcast_service(boxed))
    }

    //downcast a type-erased service to the concrete type
    fn downcast_service<T: Injectable>(&self, boxed: &ServiceBox) -> Option<Arc<T>> {
        boxed.clone().downcast::<T>().ok()
    }

    /// Resolve a service or panic if not found
    ///
    /// # Panics
    ///
    /// Panics if the service hasn't been registered.
    pub fn resolve_or_panic<T: Injectable>(&self) -> Arc<T> {
        self.resolve()
            .unwrap_or_else(|| panic!("Service {} not registered", std::any::type_name::<T>()))
    }

    /// Check if a service is registered
    pub fn contains<T: Injectable>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.services.contains_key(&type_id)
    }

    /// Get the number of registered services
    pub fn len(&self) -> usize {
        self.services.len()
    }

    /// Check if the container is empty
    pub fn is_empty(&self) -> bool {
        self.services.is_empty()
    }

    /// Clear all services from the container
    pub fn clear(&mut self) {
        self.services.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDatabase {
        connection_string: String,
    }

    impl Injectable for MockDatabase {}

    impl MockDatabase {
        fn new(conn: &str) -> Self {
            Self {
                connection_string: conn.to_string(),
            }
        }
    }

    struct MockUserService {
        db: Arc<MockDatabase>,
    }

    impl Injectable for MockUserService {}

    impl MockUserService {
        fn new(db: Arc<MockDatabase>) -> Self {
            Self { db }
        }
    }

    #[test]
    fn test_register_and_resolve() {
        let mut container = Container::new();
        let db = Arc::new(MockDatabase::new("postgres://localhost"));

        container.register(db.clone());

        let resolved: Arc<MockDatabase> = container.resolve().unwrap();
        assert_eq!(resolved.connection_string, "postgres://localhost");
    }

    #[test]
    fn test_register_factory() {
        let mut container = Container::new();

        container.register_factory(|| MockDatabase::new("sqlite::memory"));

        let resolved: Arc<MockDatabase> = container.resolve().unwrap();
        assert_eq!(resolved.connection_string, "sqlite::memory");
    }

    #[test]
    fn test_resolve_missing_service() {
        let container = Container::new();
        let result: Option<Arc<MockDatabase>> = container.resolve();
        assert!(result.is_none());
    }

    #[test]
    #[should_panic(expected = "Service")]
    fn test_resolve_or_panic() {
        let container = Container::new();
        let _: Arc<MockDatabase> = container.resolve_or_panic();
    }

    #[test]
    fn test_dependency_chain() {
        let mut container = Container::new();

        // Register database first
        let db = Arc::new(MockDatabase::new("postgres://localhost"));
        container.register(db.clone());

        // Then register service that depends on it
        let user_service = Arc::new(MockUserService::new(db));
        container.register(user_service);

        // Resolve both
        let resolved_db: Arc<MockDatabase> = container.resolve().unwrap();
        let resolved_service: Arc<MockUserService> = container.resolve().unwrap();

        assert_eq!(resolved_db.connection_string, "postgres://localhost");
        assert_eq!(
            resolved_service.db.connection_string,
            "postgres://localhost"
        );
    }

    #[test]
    fn test_contains() {
        let mut container = Container::new();
        assert!(!container.contains::<MockDatabase>());

        container.register_factory(|| MockDatabase::new("test"));
        assert!(container.contains::<MockDatabase>());
    }

    #[test]
    fn test_len_and_clear() {
        let mut container = Container::new();
        assert_eq!(container.len(), 0);
        assert!(container.is_empty());

        container.register_factory(|| MockDatabase::new("test"));
        assert_eq!(container.len(), 1);
        assert!(!container.is_empty());

        container.clear();
        assert_eq!(container.len(), 0);
        assert!(container.is_empty());
    }
}
