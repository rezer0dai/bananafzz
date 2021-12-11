/// serialization structure for argument for further POC generation
pub struct SerializationInfo {
    /// offset : where are those data positioned in argument
    ///
    /// - in case of IArg for leafs it is 0, otherwise for IArgComposite can differ
    pub offset: usize,
    /// buffer : final representation of data which can be compiled as part of source code of POC
    pub prefix: String,
}
/// every argument must be serializable in order to reproduce program / crash in POC
pub trait ISerializableArg {
    /// take mem as data buffer of given size, and print it to String (buffer) in a way that it could be compiled later on ( c++ )
    ///
    /// - further deatils check core/generator/{leaf / composite}.rs
    ///
    /// #Example
    /// ```
    /// impl ISerializableArg for TestArg {
    ///     fn serialize(&self, _: &[u8]) -> Vec<SerializationInfo> {
    ///         vec![
    ///             SerializationInfo {
    ///                 offset : 0,
    ///                 prefix : String::from("special("),
    ///             }]
    ///     }
    /// }
    /// ```
    fn serialize(&self, _: &[u8], _: &[u8]) -> Vec<SerializationInfo> {
        vec![
            SerializationInfo {
                offset : 0,
                prefix : String::from(""),
            }]
    }
}
