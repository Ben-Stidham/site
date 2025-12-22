enum ObjectType {
    File,
    Link,
    Directory,
}

pub struct FileSystemObject {
    object: ObjectType,
    name: String,
    file: Option<File>,
    directory: Option<Directory>,
    link: Option<Link>,
}

pub struct File {
    content: Vec<u8>,
}

pub struct Directory {
    sub_directories: Option<Box<Vec<FileSystemObject>>>,
}

pub struct Link {
    destination: String,
}
