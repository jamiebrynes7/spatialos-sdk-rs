use crate::worker::{
    component::{Update},
    schema::{owned::*, ArrayField, FieldId, Object, SchemaField, SchemaObjectType},
};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct ComponentUpdate(PhantomData<*mut Schema_ComponentUpdate>);

impl ComponentUpdate {
    pub fn new<U: Update>(update: &U) -> Owned<Self> {
        // Create the underlying `Schema_ComponentUpdate` and wrap it in a smart pointer.
        let mut result: Owned<Self> =
            unsafe { Owned::new(Schema_CreateComponentUpdate()) };

        // Populate the update object from the update data.
        let component_update = &mut *result;
        update.into_update(component_update);

        result
    }

    pub(crate) unsafe fn from_raw<'a>(raw: *mut Schema_ComponentUpdate) -> &'a Self {
        &*(raw as *mut _)
    }

    /// Deserializes the component update into the specified update type.
    ///
    /// Returns `None` if `U` is not the correct update type.
    pub fn deserialize<U: Update>(&self) -> Option<U> {
        // TODO: Is there any way to check that the component ID of the serialized data
        // matches the expected result type? If not, there's probably no point to
        // returning an `Option` here anymore.
        Some(U::from_update(self))
    }

    pub fn field<T: SchemaField>(&self, field: FieldId) -> Option<T::RustType> {
        T::get_update(self, field)
    }

    pub fn add_field<T: SchemaField>(&mut self, field: FieldId, value: &T::RustType) {
        T::add_update(self, field, value);
    }

    pub fn field_array<T: ArrayField>(&self, field: FieldId) -> Option<Vec<T::RustType>> {
        if T::has_update(self, field) {
            Some(self.fields().field_array::<T>(field))
        } else {
            None
        }
    }

    pub fn add_field_array<T: ArrayField>(&mut self, field: FieldId, value: &[T::RustType]) {
        self.fields_mut().add_field_array::<T>(field, value)
    }

    pub fn add_cleared(&mut self, field: FieldId) {
        unsafe {
            Schema_AddComponentUpdateClearedField(self.as_ptr(), field);
        }
    }

    pub fn field_cleared(&self, field: FieldId) -> bool {
        0 != unsafe { Schema_IsComponentUpdateFieldCleared(self.as_ptr(), field) }
    }

    pub fn event<T: SchemaObjectType>(&self, field: FieldId) -> Vec<T> {
        self.events().field::<Vec<T>>(field)
    }

    // NOTE: Due to the way the schema traits are setup, we need to take `events` as a
    // `&Vec<T>` instead of a `&[T]` (which would be more idiomatic). It's not practical
    // (and maybe not even possible) to adjust the traits to accept `&[T]` when adding a
    // list field, so we settle for suprressing the warning in this case.
    #[allow(clippy::ptr_arg)]
    pub fn add_event<T: SchemaObjectType>(&mut self, field: FieldId, events: &Vec<T>) {
        self.events_mut().add_field::<Vec<T>>(field, events);
    }

    pub(crate) fn events(&self) -> &Object {
        unsafe { Object::from_raw(Schema_GetComponentUpdateEvents(self.as_ptr())) }
    }

    pub(crate) fn events_mut(&mut self) -> &mut Object {
        unsafe { Object::from_raw_mut(Schema_GetComponentUpdateEvents(self.as_ptr())) }
    }

    /// Returns an iterator over any cleared fields included in this update.
    ///
    /// # Examples
    ///
    /// ```
    /// # // TODO: Use a pre-existing component, rather than having to define one here.
    /// # use spatialos_sdk::worker::{component::*, schema::{self, *}};
    /// # use std::{thread, sync::Arc};
    /// #
    /// # pub struct CustomComponent;
    /// #
    /// # impl Component for CustomComponent {
    /// #     const ID: ComponentId = 7777;
    /// #     type Update = CustomComponentUpdate;
    /// # }
    /// #
    /// # impl SchemaObjectType for CustomComponent {
    /// #     fn from_object(object: &Object) -> Self { Self }
    /// #     fn into_object(&self, object: &mut Object) {}
    /// # }
    /// #
    /// # pub struct CustomComponentUpdate;
    /// #
    /// # impl Update for CustomComponentUpdate {
    /// #     type Component = CustomComponent;
    /// #
    /// #     fn from_update(update: &ComponentUpdate) -> Self { Self }
    /// #     fn into_update(&self, update: &mut ComponentUpdate) {}
    /// # }
    /// # let mut update = ComponentUpdate::new(&CustomComponentUpdate);
    /// for cleared in update.cleared() {
    ///     // Clear the specified field.
    /// }
    /// ```
    pub fn cleared(&self) -> impl Iterator<Item = FieldId> + '_ {
        let count = unsafe { Schema_GetComponentUpdateClearedFieldCount(self.as_ptr()) };

        ClearedFieldIter {
            update: self,
            count,
            index: 0,
        }
    }

    pub(crate) fn fields(&self) -> &Object {
        unsafe { Object::from_raw(Schema_GetComponentUpdateFields(self.as_ptr())) }
    }

    pub(crate) fn fields_mut(&mut self) -> &mut Object {
        unsafe { Object::from_raw_mut(Schema_GetComponentUpdateFields(self.as_ptr())) }
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_ComponentUpdate {
        self as *const _ as *mut _
    }
}

impl OwnableImpl for ComponentUpdate {
    type Raw = Schema_ComponentUpdate;

    unsafe fn destroy(inst: *mut Self::Raw) {
        Schema_DestroyComponentUpdate(inst);
    }
}

unsafe impl Send for ComponentUpdate {}

struct ClearedFieldIter<'a> {
    update: &'a ComponentUpdate,
    count: u32,
    index: u32,
}

impl<'a> Iterator for ClearedFieldIter<'a> {
    type Item = FieldId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            None
        } else {
            let item = unsafe {
                Schema_IndexComponentUpdateClearedField(self.update.as_ptr(), self.index)
            };

            self.index += 1;

            Some(item)
        }
    }
}
