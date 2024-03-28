use std::{
    collections::HashMap,
    io::{Read, Write},
    path::Path,
    str,
};

use crate::{meta_data::MetaData, project::Project};
use quick_xml::{de::from_str, se::to_string};
use std::any::type_name;
use zip::ZipWriter;

const FORMAT_NAME: &str = "DAWProject exchange format";
const FILE_EXTENSION: &str = "dawproject";
const PROJECT_FILE: &str = "project.xml";
const METADATA_FILE: &str = "metadata.xml";

pub struct DawProject {
    format_name: &'static str,
    file_extension: &'static str,
    project_file: &'static str,
    metadata_file: &'static str,
}

pub enum ObjectType {
    P(Project),
    M(MetaData),
}

impl DawProject {
    pub fn new() -> Self {
        DawProject {
            format_name: FORMAT_NAME,
            file_extension: FILE_EXTENSION,
            project_file: PROJECT_FILE,
            metadata_file: METADATA_FILE,
        }
    }

    pub fn export_schema() -> Result<String, ()> {
        /*
        This probably be able to export xml schema.xsd, but it is unnecessary since we will always derive it
        from its parent project in Java. // or not?
         */
        Ok(String::new())
    }

    pub fn to_xml(object: ObjectType) -> Result<String, ()> {
        /*
        Function that takes object and returns String from that object that represents XML
         */

        match object {
            ObjectType::P(o) => match to_string(&o) {
                Ok(o_string) => Ok(o_string),
                Err(_) => Err(()),
            },
            ObjectType::M(o) => match to_string(&o) {
                Ok(o_string) => Ok(o_string),
                Err(_) => Err(()),
            },
        }
    }

    pub fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    pub fn project_from_xml(xml_string: String) -> Result<Project, ()> {
        // Essentially takes Project or
        match from_str(&xml_string) {
            Ok(project) => Ok(project),
            Err(_) => Err(()),
        }
    }

    pub fn metadata_from_xml(xml_string: String) -> Result<MetaData, ()> {
        // Essentially takes Project or
        match from_str(&xml_string) {
            Ok(project) => Ok(project),
            Err(_) => Err(()),
        }
    }

    pub fn create_context() -> Result<(), ()> {
        /*
        This functions is creating some JABCONTEXT, whatever it is, unnecessary I think
         */
        Ok(())
    }

    pub fn save_xml(project: Project, path: &Path) -> Result<(), ()> {
        use std::fs;
        /*
          This is purely for file save handling
        */
        let project_xml: String;

        match DawProject::to_xml(ObjectType::P(project)) {
            Ok(x) => {
                project_xml = x;
            }
            Err(_) => return Err(()),
        };

        match fs::write(path, project_xml) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn validate(_project: Project) -> Result<(), ()> {
        /*
        This `validate` function takes a `Project` object, converts it to XML using `toXML`,
        then validates the XML against an XML schema for `Project` objects, throwing an `IOException`
        if validation fails due to a JAXB or SAX exception.

        So it is safeguard
        */
        Ok(())
    }

    pub fn save(
        project: Project,
        meta_data: MetaData,
        _embedded_files: HashMap<&Path, String>,
        zip_file_path: &Path,
    ) -> Result<(), ()> {
        let file = match std::fs::File::create(zip_file_path) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        let mut zip_writer = zip::ZipWriter::new(file);

        let project_xml = match Self::to_xml(ObjectType::P(project)) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        let meta_data_xml = match Self::to_xml(ObjectType::M(meta_data)) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        match Self::add_bytes_to_zip(&mut zip_writer, project_xml.as_bytes(), PROJECT_FILE) {
            Ok(()) => (),
            Err(_) => return Err(()),
        }

        match Self::add_bytes_to_zip(&mut zip_writer, meta_data_xml.as_bytes(), METADATA_FILE) {
            Ok(()) => (),
            Err(_) => return Err(()),
        }

        // for (p, s) in embedded_files { //
        //     match Self::add_bytes_to_zip(&mut zip_writer, , &s) {
        //         Ok(_) => (),
        //         Err(_) => return Err(()),
        //     }
        // }

        Ok(())
    }

    fn add_bytes_to_zip<W: std::io::Write + std::io::Seek>(
        zip_writer: &mut ZipWriter<W>,
        content: &[u8],
        file_name: &str,
    ) -> Result<(), ()> {
        /*
           final ZipEntry entry = new        ZipEntry(path);
        zos.putNextEntry(entry);
        zos.write(data);
        zos.closeEntry();
           */
        let name = format!("./{}", file_name);

        match zip_writer.start_file(name, Default::default()) {
            Ok(_) => (),
            Err(_) => return Err(()),
        }

        match zip_writer.write_all(content) {
            Ok(_) => (),
            Err(_) => return Err(()),
        }

        Ok(())
    }

    pub fn add_file_to_zip_from_str<W: std::io::Write + std::io::Seek>(
        zip_writer: &mut ZipWriter<W>,
        content: &str,
        file_name: &str,
    ) -> Result<(), ()> {
        /*
           final ZipEntry entry = new        ZipEntry(path);
        zos.putNextEntry(entry);
        zos.write(data);
        zos.closeEntry();
           */
        let name = format!("./{}", file_name);

        match zip_writer.start_file(name, Default::default()) {
            Ok(_) => (),
            Err(_) => return Err(()),
        }

        match zip_writer.write_all(content.as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err(()),
        }

        Ok(())
    }

    pub fn strip_bom() {
        /*
        This stripBom function takes an InputStream, detects and removes any Byte Order Marks (BOMs) present in the stream,
        then returns an InputStreamReader using the appropriate Charset,
        defaulting to UTF-8 if no BOM is found or if the BOM is not recognized as UTF-8,
        UTF-16LE, or UTF-16BE. If the BOM is not recognized, it throws an IOException with the message
        "The charset is not supported."
        */
    }

    pub fn load_project(fname: &Path) -> Result<Project, ()> {
        let zip_file = std::fs::File::open(fname).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

        let mut contents = String::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let _out_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if file.name() == PROJECT_FILE {
                match file.read_to_string(&mut contents) {
                    Ok(_v) => (),
                    Err(_) => return Err(()),
                };
            }
        }

        let project: Project = match from_str(&contents) {
            Ok(p) => p,
            Err(_) => return Err(()),
        };

        Ok(project)
    }

    pub fn load_metadata(fname: &Path) -> Result<MetaData, ()> {
        let zip_file = std::fs::File::open(fname).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

        let mut contents = String::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let _out_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if file.name() == PROJECT_FILE {
                match file.read_to_string(&mut contents) {
                    Ok(_v) => (),
                    Err(_) => return Err(()),
                };
            }
        }

        let metadata: MetaData = match from_str(&contents) {
            Ok(p) => p,
            Err(_) => return Err(()),
        };

        Ok(metadata)
    }

    pub fn stream_embedded(_file: &Path, _embedded_path: String) -> Result<(), ()> {
        /*
             final ZipFile zipFile = new ZipFile (file);
        final ZipEntry entry = zipFile.getEntry (embeddedPath);
        final InputStream zipInputStream = zipFile.getInputStream (entry);

        // Ensure that both the stream and the ZIP file gets closed
        return new InputStream ()
        {
            @Override
            public int read () throws IOException
            {
                return zipInputStream.read ();
            }


            @Override
            public void close () throws IOException
            {
                zipInputStream.close ();
                zipFile.close ();
            }
        };

          */
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DawProject;
    use std::io::Read;
    use std::str::FromStr;
    // Write tests and code for saving the projects then use it in general tests and that's it

    #[test]
    fn create_zip_with_text_and_load_test() -> Result<(), String> {
        let zip_file_path = "./test.zip";
        let file = match std::fs::File::create(zip_file_path) {
            Ok(f) => f,
            Err(_) => return Err("Creation of file fails".to_string()),
        };

        let mut zip_writer = zip::ZipWriter::new(file);

        let contents = String::from_str("This is just the test").unwrap();
        let filename = "test.dawproject";

        match DawProject::add_file_to_zip_from_str(&mut zip_writer, &contents, filename) {
            Ok(_) => (),
            Err(_) => return Err("Add file to zip fails".to_string()),
        }

        match zip_writer.finish() {
            Ok(_) => (),
            Err(_) => return Err("First finish fails".to_string()),
        }

        let zip_file = std::fs::File::open(zip_file_path).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

        let mut contents_read = String::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let _out_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if file.name().contains(filename) {
                match file.read_to_string(&mut contents_read) {
                    Ok(_) => println!("File {}", file.name()),
                    Err(_) => return Err("Reading to string fails".to_string()),
                };
            }
        }

        assert_eq!(contents, contents_read);

        Ok(())
    }
}
