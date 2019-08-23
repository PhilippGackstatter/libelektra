use crate::ReadableKey;
use crate::{KeySet, StringKey, WriteableKey};
use std::ptr::NonNull;
use KDBError::*;
use LogicalError::*;
use PermanentError::*;
use ResourceError::*;
use ValidationError::*;

#[derive(Debug)]
pub struct KDB {
    ptr: NonNull<elektra_sys::KDB>,
    _marker: std::marker::PhantomData<elektra_sys::KDB>,
}

impl Drop for KDB {
    fn drop(&mut self) {
        println!("Drop {:?}", self);
        let mut err_key = StringKey::new_empty();
        unsafe {
            elektra_sys::kdbClose(self.as_ptr(), err_key.as_ptr());
        }
    }
}

impl KDB {
    /// Opens the session with the Key database.
    pub fn open<'a>() -> Result<Self, KDBError<'a>> {
        let mut error_key = StringKey::new_empty();
        let kdb_ptr = unsafe { elektra_sys::kdbOpen(error_key.as_ptr()) };

        if kdb_ptr.is_null() {
            Err(map_kdb_error(error_key))
        } else {
            Ok(KDB {
                ptr: NonNull::new(kdb_ptr).unwrap(),
                _marker: std::marker::PhantomData,
            })
        }
    }

    /// Retrieve keys in an atomic and universal way.
    /// Note that the provided keyset is modified and contains the result.
    /// The return value is true, if the keys were successfully retrieved
    /// and false if there were no changes to the keyset.
    pub fn get<'a>(
        &mut self,
        keyset: &mut KeySet,
        key: &mut StringKey<'a>,
    ) -> Result<bool, KDBError<'a>> {
        let ret_val = unsafe { elektra_sys::kdbGet(self.as_ptr(), keyset.as_ptr(), key.as_ptr()) };

        if ret_val == 1 {
            Ok(true)
        } else if ret_val == 0 {
            Ok(false)
        } else {
            Err(map_kdb_error(key.duplicate()))
        }
    }

    /// Set keys in an atomic and universal way.
    /// The return value is true on success,
    /// and false if there were no changes to the KDB.
    /// # Notes
    /// You have to call [`get`] with `keyset` first.
    ///
    /// [`get`]: #method.get
    pub fn set<'a>(
        &mut self,
        keyset: &mut KeySet,
        key: &mut StringKey<'a>,
    ) -> Result<bool, KDBError<'a>> {
        let ret_val = unsafe { elektra_sys::kdbSet(self.as_ptr(), keyset.as_ptr(), key.as_ptr()) };

        if ret_val == 1 {
            Ok(true)
        } else if ret_val == 0 {
            Ok(false)
        } else {
            Err(map_kdb_error(key.duplicate()))
        }
    }

    /// This method can be used the given KDB handle meets certain clauses, specified in contract.
    /// The return value is true on success,
    /// and false if clauses of the contract are unmet.
    pub fn ensure<'a>(
        &mut self,
        keyset: &mut KeySet,
        key: &mut StringKey<'a>,
    ) -> Result<bool, KDBError<'a>> {
        let ret_val =
            unsafe { elektra_sys::kdbEnsure(self.as_ptr(), keyset.as_ptr(), key.as_ptr()) };

        if ret_val == 0 {
            Ok(true)
        } else if ret_val == 1 {
            Ok(false)
        } else {
            Err(map_kdb_error(key.duplicate()))
        }
    }

    pub fn as_ptr(&mut self) -> *mut elektra_sys::KDB {
        self.ptr.as_ptr()
    }
}

impl AsRef<elektra_sys::KDB> for KDB {
    fn as_ref(&self) -> &elektra_sys::KDB {
        unsafe { self.ptr.as_ref() }
    }
}

// const ELEKTRA_ERROR_RESOURCE: &str = "C01100";
const ELEKTRA_ERROR_OUT_OF_MEMORY: &str = "C01110";
// const ELEKTRA_ERROR_INSTALLATION: &str = "C01200";
const ELEKTRA_ERROR_INTERNAL: &str = "C01310";
const ELEKTRA_ERROR_INTERFACE: &str = "C01320";
const ELEKTRA_ERROR_PLUGIN_MISBEHAVIOR: &str = "C01330";
const ELEKTRA_ERROR_CONFLICTING_STATE: &str = "C02000";
const ELEKTRA_ERROR_VALIDATION_SYNTACTIC: &str = "C03100";
const ELEKTRA_ERROR_VALIDATION_SEMANTIC: &str = "C03200";

pub enum KDBError<'a> {
    Permanent(PermanentError<'a>),
    ConflictingState(KDBErrorWrapper<'a>),
    Validation(ValidationError<'a>),
}

pub enum PermanentError<'a> {
    Resource(ResourceError<'a>),
    Logical(LogicalError<'a>),
    Installation(KDBErrorWrapper<'a>),
}

pub enum LogicalError<'a> {
    Internal(KDBErrorWrapper<'a>),
    Interface(KDBErrorWrapper<'a>),
    PluginMisbehavior(KDBErrorWrapper<'a>),
}

pub enum ResourceError<'a> {
    OutOfMemory(KDBErrorWrapper<'a>),
}

pub enum ValidationError<'a> {
    Syntactic(KDBErrorWrapper<'a>),
    Semantic(KDBErrorWrapper<'a>),
}

#[derive(Debug)]
pub struct KDBErrorWrapper<'a> {
    error_key: StringKey<'a>,
}

impl<'a> KDBErrorWrapper<'a> {
    /// Constructs a new KDBErrorWrapper from a StringKey.
    /// Only pass keys where the metakeys error/* are set.
    pub fn new(error_key: StringKey) -> KDBErrorWrapper {
        KDBErrorWrapper { error_key }
    }

    // TODO: For which of these error/* can we be sure that they exist?

    /// Returns the error number.
    pub fn number(&self) -> String {
        self.error_key
            .meta("error/number")
            .unwrap()
            .value()
            .to_owned()
            .to_string()
    }

    /// Returns the error reason.
    pub fn reason(&self) -> String {
        self.error_key
            .meta("error/reason")
            .unwrap()
            .value()
            .to_owned()
            .to_string()
    }

    /// Returns the module where the error occured.
    pub fn module(&self) -> String {
        self.error_key
            .meta("error/module")
            .unwrap()
            .value()
            .to_owned()
            .to_string()
    }

    /// Returns a description of the error.
    pub fn description(&self) -> String {
        self.error_key
            .meta("error/description")
            .unwrap()
            .value()
            .to_owned()
            .to_string()
    }

    /// Returns the source file from where the error information comes.
    pub fn file(&self) -> String {
        self.error_key
            .meta("error/file")
            .unwrap()
            .value()
            .to_owned()
            .to_string()
    }

    /// Returns the the exact line of that source file.
    pub fn line(&self) -> String {
        self.error_key
            .meta("error/line")
            .unwrap()
            .value()
            .to_owned()
            .to_string()
    }
    // TODO: key is not the error_key, but the key that the keysets internal cursor points to, but this is not accessible here
    pub fn to_error_message(&self) -> String {
        format!("Sorry, module {module} issued error {error_number}:\n{description}: Validation of key \"{key}\" with string \"{string}\" failed.", 
            module = self.module(),
            error_number = self.number(),
            description = self.description(),
            key = self.error_key.name(),
            string = self.error_key.value())
    }
}

impl<'a> std::fmt::Display for KDBErrorWrapper<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_error_message())
    }
}

impl<'a> std::error::Error for KDBErrorWrapper<'a> {}

pub fn map_kdb_error(error_key: StringKey) -> KDBError {
    let err_num_key_res = error_key.meta("error/number");
    if let Ok(err_num_key) = err_num_key_res {
        let err_wrapper = KDBErrorWrapper::new(error_key.duplicate());

        match err_num_key.value().to_owned().to_string().as_str() {
            ELEKTRA_ERROR_OUT_OF_MEMORY => {
                return Permanent(Resource(OutOfMemory(err_wrapper)));
            }
            ELEKTRA_ERROR_INTERNAL => {
                return Permanent(Logical(Internal(err_wrapper)));
            }
            ELEKTRA_ERROR_INTERFACE => {
                return Permanent(Logical(Interface(err_wrapper)));
            }
            ELEKTRA_ERROR_PLUGIN_MISBEHAVIOR => {
                return Permanent(Logical(PluginMisbehavior(err_wrapper)));
            }
            ELEKTRA_ERROR_CONFLICTING_STATE => {
                return ConflictingState(err_wrapper);
            }
            ELEKTRA_ERROR_VALIDATION_SYNTACTIC => {
                return Validation(Syntactic(err_wrapper));
            }
            ELEKTRA_ERROR_VALIDATION_SEMANTIC => {
                return Validation(Semantic(err_wrapper));
            }
            _ => {
                unreachable!();
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_open_kdb() {
        let result = KDB::open();
        assert!(result.is_ok());
    }
}