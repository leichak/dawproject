use std::{
    collections::HashMap,
    io::{Read, Write},
    path::Path,
    str,
};

use quick_xml::{de::from_str, se::to_string};
use zip::ZipWriter;

use crate::{meta_data::MetaData, project::Project};

const FORMAT_NAME: &'static str = "DAWProject exchange format";
const FILE_EXTENSION: &'static str = "dawproject";
const PROJECT_FILE: &'static str = "project.xml";
const METADATA_FILE: &'static str = "metadata.xml";

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
        from its parent project in Java.
         */
        Ok(String::new())
    }

    pub fn to_xml(object: ObjectType) -> Result<String, ()> {
        /*
        I think it just takes project and saves it as xml file
        So we can pass for example cloned object, generate String from it
        and save it as xml file.
         */

        match object {
            ObjectType::P(o) => {
                match to_string(&o) {
                    Ok(o_string) => return Ok(o_string),
                    Err(_) => return Err(()),
                };
            }
            ObjectType::M(o) => {
                match to_string(&o) {
                    Ok(o_string) => return Ok(o_string),
                    Err(_) => return Err(()),
                };
            }
        };
    }

    pub fn from_xml(xml_string: String) -> Result<Project, ()> {
        match from_str(&xml_string) {
            Ok(project) => return Ok(project),
            Err(_) => return Err(()),
        };
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

          String projectXML = toXML(project);
        FileOutputStream fileOutputStream = new FileOutputStream(file);
        fileOutputStream.write(projectXML.getBytes(StandardCharsets.UTF_8));
        fileOutputStream.close();
           */

        // For now our solution will replace old file
        let project_xml: String;

        match DawProject::to_xml(ObjectType::P(project)) {
            Ok(x) => {
                project_xml = x;
            }
            Err(_) => return Err(()),
        };

        match fs::write(path, project_xml) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(()),
        }
    }

    pub fn validate(project: Project) -> Result<(), ()> {
        /*
        Should be some sort of validation mechanism which checks whether
        xml-ed project struct work accordingly to schema
         */
        // Irrelevant so fa until I will be able to export schema

        Ok(())
    }

    pub fn save(
        project: Project,
        meta_data: MetaData,
        embedded_files: HashMap<&Path, String>,
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

        match Self::add_bytes_to_zip(&mut zip_writer, &project_xml.as_bytes(), PROJECT_FILE) {
            Ok(()) => (),
            Err(_) => return Err(()),
        }

        match Self::add_bytes_to_zip(&mut zip_writer, &meta_data_xml.as_bytes(), METADATA_FILE) {
            Ok(()) => (),
            Err(_) => return Err(()),
        }

        // for (p, s) in embedded_files {
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

    fn add_str_to_zip<W: std::io::Write + std::io::Seek>(
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
        // Thats just checks for byte order - I think that is irrelevant in that case
        /* I do not know that that is
        BOMInputStream bomInputStream = new BOMInputStream(inputStream ,
        ByteOrderMark.UTF_8, ByteOrderMark.UTF_16LE, ByteOrderMark.UTF_16BE, ByteOrderMark.UTF_32LE, ByteOrderMark.UTF_32BE);
        Charset charset;
        if(!bomInputStream.hasBOM()) charset = StandardCharsets.UTF_8;
        else if(bomInputStream.hasBOM(ByteOrderMark.UTF_8)) charset = StandardCharsets.UTF_8;
        else if(bomInputStream.hasBOM(ByteOrderMark.UTF_16LE)) charset = StandardCharsets.UTF_16LE;
        else if(bomInputStream.hasBOM(ByteOrderMark.UTF_16BE)) charset = StandardCharsets.UTF_16BE;
        else { throw new IOException("The charset is not supported.");}

        return new InputStreamReader(bomInputStream, charset);
           */
    }

    pub fn load_project(fname: &Path) -> Result<Project, ()> {
        let zip_file = std::fs::File::open(fname).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

        let mut contents = String::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let out_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if file.name() == PROJECT_FILE {
                match file.read_to_string(&mut contents) {
                    Ok(v) => (),
                    Err(_) => return Err(()),
                };
            }
        }

        let mut project: Project = match from_str(&contents) {
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
            let out_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if file.name() == PROJECT_FILE {
                match file.read_to_string(&mut contents) {
                    Ok(v) => (),
                    Err(_) => return Err(()),
                };
            }
        }

        let mut metadata: MetaData = match from_str(&contents) {
            Ok(p) => p,
            Err(_) => return Err(()),
        };

        Ok(metadata)
    }

    pub fn stream_embedded(file: &Path, embedded_path: String) -> Result<(), ()> {
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
#[test]
fn test() {}
