#[derive(Debug)]
pub struct LookupInSpec {
    path: String,
    command_type: SubdocLookupCommandType,
    xattr: bool,
}

impl LookupInSpec {
    pub fn get<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        LookupInSpec {
            path: path.into(),
            command_type: SubdocLookupCommandType::Get,
            xattr: false,
        }
    }

    pub fn get_full_document() -> Self
    {
        LookupInSpec {
            path: "".into(),
            command_type: SubdocLookupCommandType::GetDoc,
            xattr: false,
        }
    }

    pub fn count<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        LookupInSpec {
            path: path.into(),
            command_type: SubdocLookupCommandType::Count,
            xattr: false,
        }
    }

    pub fn exists<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        LookupInSpec {
            path: path.into(),
            command_type: SubdocLookupCommandType::Exists,
            xattr: false,
        }
    }

    pub fn xattr(mut self) -> Self {
        self.xattr = true;
        self
    }

    pub(crate) fn command_type(&self) -> &SubdocLookupCommandType {
        &self.command_type
    }

    pub(crate) fn path(&self) -> &String {
        &self.path
    }
}

#[derive(Debug)]
pub enum SubdocLookupCommandType {
    Get,
    Exists,
    Count,
    GetDoc,
}
