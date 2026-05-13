use crate::*;

/// Implementation of `Default` trait for `Context`.
impl Default for Context {
    /// Creates a default `Context` instance.
    ///
    /// # Returns
    ///
    /// - `Context` - A new context with default values and a static default server.
    #[inline(always)]
    fn default() -> Self {
        Self {
            request: Request::default(),
            response: Response::default(),
            route_params: RouteParams::default(),
            attributes: ThreadSafeAttributeStore::default(),
        }
    }
}

/// Implementation of `PartialEq` trait for `Context`.
impl PartialEq for Context {
    /// Compares two `Context` instances for equality.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The first `Context` instance.
    /// - `&Self` - The second `Context` instance.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the instances are equal, otherwise false.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.get_request() == other.get_request()
            && self.get_response() == other.get_response()
            && self.get_route_params() == other.get_route_params()
            && self.get_attributes().len() == other.get_attributes().len()
    }
}

/// Implementation of `Eq` trait for `Context`.
impl Eq for Context {}

/// Implementation of `From` trait for converting `usize` address into `&Context`.
impl From<usize> for &'static Context {
    /// Converts a memory address into a reference to `Context`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Context` instance.
    ///
    /// # Returns
    ///
    /// - `&'static Context` - A reference to the `Context` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Context` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> &'static Context {
        unsafe { &*(address as *const Context) }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&mut Context`.
impl<'a> From<usize> for &'a mut Context {
    /// Converts a memory address into a mutable reference to `Context`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Context` instance.
    ///
    /// # Returns
    ///
    /// - `&mut Context` - A mutable reference to the `Context` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Context` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> &'a mut Context {
        unsafe { &mut *(address as *mut Context) }
    }
}

/// Implementation of `From` trait for converting `&Context` into `usize` address.
impl From<&Context> for usize {
    /// Converts a reference to `Context` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The reference to the `Context` instance.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the `Context` instance.
    #[inline(always)]
    fn from(ctx: &Context) -> Self {
        ctx as *const Context as usize
    }
}

/// Implementation of `From` trait for converting `&mut Context` into `usize` address.
impl From<&mut Context> for usize {
    /// Converts a mutable reference to `Context` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The mutable reference to the `Context` instance.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the `Context` instance.
    #[inline(always)]
    fn from(ctx: &mut Context) -> Self {
        ctx as *mut Context as usize
    }
}

/// Implementation of `AsRef` trait for `Context`.
impl AsRef<Context> for Context {
    /// Converts `&Context` to `&Context` via memory address conversion.
    ///
    /// # Returns
    ///
    /// - `&Context` - A reference to the `Context` instance.
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Implementation of `AsMut` trait for `Context`.
impl AsMut<Context> for Context {
    /// Converts `&mut Context` to `&mut Context` via memory address conversion.
    ///
    /// # Returns
    ///
    /// - `&mut Context` - A mutable reference to the `Context` instance.
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Implementation of `Lifetime` trait for `Context`.
impl Lifetime for Context {
    /// Converts a reference to the context into a `'static` reference.
    ///
    /// # Returns
    ///
    /// - `&'static Self`: A reference to the context with a `'static` lifetime.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Self` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    unsafe fn leak(&self) -> &'static Self {
        let address: usize = self.into();
        address.into()
    }

    /// Converts a reference to the context into a `'static` mutable reference.
    ///
    /// # Returns
    ///
    /// - `&'static mut Self`: A mutable reference to the context with a `'static` lifetime.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Self` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    unsafe fn leak_mut(&self) -> &'static mut Self {
        let address: usize = self.into();
        address.into()
    }
}

/// Implementation of methods for `Context` structure.
impl Context {
    /// Attempts to retrieve a specific route parameter by its name.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The name of the route parameter to retrieve.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The value of the route parameter if it exists.
    #[inline(always)]
    pub fn try_get_route_param<T>(&self, name: T) -> Option<String>
    where
        T: AsRef<str>,
    {
        self.get_route_params().get(name.as_ref()).cloned()
    }

    /// Retrieves a specific route parameter by its name, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The name of the route parameter to retrieve.
    ///
    /// # Returns
    ///
    /// - `String` - The value of the route parameter if it exists.
    ///
    /// # Panics
    ///
    /// - If the route parameter is not found.
    #[inline(always)]
    pub fn get_route_param<T>(&self, name: T) -> String
    where
        T: AsRef<str>,
    {
        self.try_get_route_param(name).unwrap()
    }

    /// Attempts to retrieve a specific attribute by its key, casting it to the specified type.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the attribute to retrieve.
    ///
    /// # Returns
    ///
    /// - `Option<V>` - The attribute value if it exists and can be cast to the specified type.
    #[inline(always)]
    pub fn try_get_attribute<V>(&self, key: impl AsRef<str>) -> Option<V>
    where
        V: AnySendSyncClone,
    {
        self.get_attributes()
            .get(&Attribute::External(key.as_ref().to_owned()).to_string())
            .and_then(|arc| arc.downcast_ref::<V>())
            .cloned()
    }

    /// Retrieves a specific attribute by its key, casting it to the specified type, panicking if not found.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the attribute to retrieve.
    ///
    /// # Returns
    ///
    /// - `AnySendSyncClone` - The attribute value if it exists and can be cast to the specified type.
    ///
    /// # Panics
    ///
    /// - If the attribute is not found.
    #[inline(always)]
    pub fn get_attribute<V>(&self, key: impl AsRef<str>) -> V
    where
        V: AnySendSyncClone,
    {
        self.try_get_attribute(key).unwrap()
    }

    /// Sets an attribute in the context.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the attribute to set.
    /// - `AnySendSyncClone` - The value of the attribute.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A reference to the modified context.
    #[inline(always)]
    pub fn set_attribute<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AnySendSyncClone,
    {
        self.get_mut_attributes().insert(
            Attribute::External(key.as_ref().to_owned()).to_string(),
            Arc::new(value),
        );
        self
    }

    /// Removes an attribute from the context.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the attribute to remove.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A reference to the modified context.
    #[inline(always)]
    pub fn remove_attribute<K>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
        self.get_mut_attributes()
            .remove(&Attribute::External(key.as_ref().to_owned()).to_string());
        self
    }

    /// Clears all attributes from the context.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A reference to the modified context.
    #[inline(always)]
    pub fn clear_attribute(&mut self) -> &mut Self {
        self.get_mut_attributes().clear();
        self
    }

    /// Retrieves an internal framework attribute.
    ///
    /// # Arguments
    ///
    /// - `InternalAttribute` - The internal attribute key to retrieve.
    ///
    /// # Returns
    ///
    /// - `Option<V>` - The attribute value if it exists and can be cast to the specified type.
    #[inline(always)]
    fn try_get_internal_attribute<V>(&self, key: InternalAttribute) -> Option<V>
    where
        V: AnySendSyncClone,
    {
        self.get_attributes()
            .get(&Attribute::Internal(key).to_string())
            .and_then(|arc| arc.downcast_ref::<V>())
            .cloned()
    }

    /// Retrieves an internal framework attribute.
    ///
    /// # Arguments
    ///
    /// - `InternalAttribute` - The internal attribute key to retrieve.
    ///
    /// # Returns
    ///
    /// - `AnySendSyncClone` - The attribute value if it exists and can be cast to the specified type.
    ///
    /// # Panics
    ///
    /// - If the attribute is not found.
    #[inline(always)]
    fn get_internal_attribute<V>(&self, key: InternalAttribute) -> V
    where
        V: AnySendSyncClone,
    {
        self.try_get_internal_attribute(key).unwrap()
    }

    /// Sets an internal framework attribute.
    ///
    /// # Arguments
    ///
    /// - `InternalAttribute` - The internal attribute key to set.
    /// - `AnySendSyncClone` - The value of the attribute.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A reference to the modified context.
    #[inline(always)]
    fn set_internal_attribute<V>(&mut self, key: InternalAttribute, value: V) -> &mut Self
    where
        V: AnySendSyncClone,
    {
        self.get_mut_attributes()
            .insert(Attribute::Internal(key).to_string(), Arc::new(value));
        self
    }

    /// Stores panic data for the current task context.
    ///
    /// # Arguments
    ///
    /// - `PanicData` - The panic data specific to the current task.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Reference to the modified context for method chaining.
    #[inline(always)]
    pub(crate) fn set_task_panic(&mut self, panic_data: PanicData) -> &mut Self {
        self.set_internal_attribute(InternalAttribute::TaskPanicData, panic_data)
    }

    /// Retrieves panic data associated with the current task.
    ///
    /// # Returns
    ///
    /// - `Option<PanicData>` - Task panic data if a panic was caught during execution.
    #[inline(always)]
    pub fn try_get_task_panic_data(&self) -> Option<PanicData> {
        self.try_get_internal_attribute(InternalAttribute::TaskPanicData)
    }

    /// Retrieves panic data associated with the current task.
    ///
    /// # Returns
    ///
    /// - `PanicData` - Task panic data if available.
    ///
    /// # Panics
    ///
    /// - If no task panic data is found.
    #[inline(always)]
    pub fn get_task_panic_data(&self) -> PanicData {
        self.get_internal_attribute(InternalAttribute::TaskPanicData)
    }

    /// Sets the request error information for the context.
    ///
    /// # Arguments
    ///
    /// - `RequestError` - The request error information to store.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A reference to the modified context.
    #[inline(always)]
    pub(crate) fn set_request_error_data(&mut self, request_error: RequestError) -> &mut Self {
        self.set_internal_attribute(InternalAttribute::RequestErrorData, request_error)
    }

    /// Retrieves request error information if an error occurred during handling.
    ///
    /// # Returns
    ///
    /// - `Option<RequestError>` - The request error information if an error was caught.
    #[inline(always)]
    pub fn try_get_request_error_data(&self) -> Option<RequestError> {
        self.try_get_internal_attribute(InternalAttribute::RequestErrorData)
    }

    /// Retrieves request error information if an error occurred during handling.
    ///
    /// # Returns
    ///
    /// - `RequestError` - The request error information if an error was caught.
    ///
    /// # Panics
    ///
    /// - If the request error information is not found.
    #[inline(always)]
    pub fn get_request_error_data(&self) -> RequestError {
        self.get_internal_attribute(InternalAttribute::RequestErrorData)
    }
}
