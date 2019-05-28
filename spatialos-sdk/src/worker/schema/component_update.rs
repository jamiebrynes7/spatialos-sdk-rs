use crate::worker::{
    component::{Component, ComponentId, Update},
    schema::{owned::*, FieldId, Object, SchemaField},
};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct ComponentUpdate(PhantomData<*mut Schema_ComponentUpdate>);

impl ComponentUpdate {
    pub fn new<U: Update>(update: &U) -> Owned<Self> {
        // Create the underlying `Schema_ComponentUpdate` and wrap it in a smart pointer.
        let mut result: Owned<Self> =
            unsafe { Owned::new(Schema_CreateComponentUpdate(U::Component::ID)) };

        // Populate the update object from the update data.
        let component_update = &mut *result;
        update.into_update(component_update);

        result
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentUpdateComponentId(self.as_ptr()) }
    }

    pub fn field<T: SchemaField>(&self, field: FieldId) -> Option<T::RustType> {
        T::get_update(self, field)
    }

    pub fn add<T: SchemaField>(&mut self, field: FieldId, value: &T::RustType) {
        T::add_update(self, field, value);
    }

    pub fn add_cleared(&mut self, field: FieldId) {
        unsafe {
            Schema_AddComponentUpdateClearedField(self.as_ptr(), field);
        }
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

    fn as_ptr(&self) -> *mut Schema_ComponentUpdate {
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
