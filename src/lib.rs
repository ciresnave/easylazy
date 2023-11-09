//! EasyLazy - Lazy initialization made easy
//! 
//! Looking at the available options for lazy initialization, I found that the most
//! required jumping through odd, unnecessary hoops.  EasyLazy aims to make lazy
//! initialization of a variable lightweight and easy.
//! 
//! EasyLazy has only 3 requirements:
//! - T must implement Clone
//! - T must implement Default with a cheap operation
//! - The Lazy variable must be mutable so that it can be initialized
//! 
//! Lazy is Send & Sync so long as T is Send & Sync.  Please make sure that your
//! calculation is also thread safe before assuming YOUR Lazy variable is Send &
//! Sync.

#![no_std]

extern crate alloc;
use alloc::boxed::Box;

/// A lazy variable of type T
pub struct Lazy<T>
where T: Clone + Default
{
    /// The value of the lazy variable
    value: Option<T>,
    /// The function to call to calculate the value of the lazy variable
    calculation: Box<dyn Fn() -> T>,
}

unsafe impl<T> Send for Lazy<T>
where T: Clone + Default + Send
{
    // Nothing to do here
}

unsafe impl<T> Sync for Lazy<T>
where T: Clone + Default + Sync
{
    // Nothing to do here
}

impl<T> Lazy<T>
where T: Clone + Default
{
    /// Create a new Lazy variable
    /// 
    /// # Arguments
    /// * `calculation` - The function to call to calculate the value of the lazy variable
    /// 
    /// # Returns
    /// A new Lazy variable
    /// 
    /// # Example
    /// ```
    /// use easylazy::Lazy;
    /// let mut my_lazy_variable = Lazy::new(Box::new(|| 10));
    /// // my_lazy_variable is uninitialized here
    /// assert_eq!(my_lazy_variable.get(), 10);
    /// ```
    pub fn new(calculation: Box<dyn Fn() -> T>) -> Self 
    {
        Self 
        {
            value: None,
            calculation,
        }
    }

    /// Get the value of the lazy variable
    /// 
    /// # Returns
    /// The value of the lazy variable
    /// 
    /// # Example
    /// ```
    /// use easylazy::Lazy;
    /// let mut my_lazy_variable = Lazy::new(Box::new(|| 10));
    /// // my_lazy_variable is uninitialized here
    /// assert_eq!(my_lazy_variable.get(), 10);
    /// ```
    pub fn get(&mut self) -> T 
    {
        if self.value.is_some() 
        {
            return self.value.clone().unwrap()
        }
        self.value = Some((self.calculation)());
        self.value.clone().unwrap()
    }

    /// Get a mutable reference to the value of the lazy variable
    /// 
    /// # Returns
    /// A mutable reference to the value of the lazy variable
    /// 
    /// # Example
    /// ```
    /// use easylazy::Lazy;
    /// let mut my_lazy_variable = Lazy::new(Box::new(|| 2 + 2));
    /// // my_lazy_variable is uninitialized here
    /// *my_lazy_variable.get_mut() = 42;
    /// assert_eq!(my_lazy_variable.get(), 42);
    /// ```
    pub fn get_mut(&mut self) -> &mut T 
    {
        if self.value.is_none()
        {
            self.value = Some((self.calculation)());
        }
        if let Some(value) = self.value.as_mut()
        {
            value
        }
        else
        {
            panic!("Value inside Lazy is None after being set to Some(value)"); 
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn get_test() 
    {
        let mut my_lazy_variable = Lazy::new(Box::new(|| 2 + 2));
        assert_eq!(my_lazy_variable.get(), 4);
    }

    #[test]
    fn get_mut_test()
    {
        let mut my_lazy_variable = Lazy::new(Box::new(|| 2 + 2));
        let value = my_lazy_variable.get_mut();
        *value = 10;
        assert_eq!(my_lazy_variable.get(), 10);
    }
}
