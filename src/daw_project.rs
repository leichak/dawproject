use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

use zip::ZipWriter;

use crate::{
    meta_data::{self, MetaData},
    project::Project,
};

const FORMAT_NAME: &'static str = "DAWProject exchange format";
const FILE_EXTENSION: &'static str = "dawproject";
const PROJECT_FILE: &'static str = "project.xml";
const METADATA_FILE: &'static str = "metadata.xml";

struct DawProject {
    format_name: &'static str,
    file_extension: &'static str,
    project_file: &'static str,
    metadata_file: &'static str,
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

    pub fn export_schema() -> Result<(), ()> {
        /*
        This probably be able to export xml schema.xsd, but it is unnecessary since we will always derive it
        from its parent project in Java.
         */
        Ok(())
    }

    pub fn to_xml(project: Project) -> Result<String, ()> {
        /*
        I think it just takes project and saves it as xml file
        So we can pass for example cloned object, generate String from it
        and save it as xml file.
         */

        use quick_xml::se::to_string;

        match to_string(&project) {
            Ok(object) => return Ok(object),
            Err(_) => return Err(()),
        };
    }

    pub fn create_context() -> Result<(), ()> {
        /*
        This functions is creating some JABCONTEXT, whatever it is, unnecessary I think
         */
        Ok(())
    }

    pub fn from_xml(xml_string: String) -> Result<Project, ()> {
        use quick_xml::de::from_str;

        match from_str(&xml_string) {
            Ok(project) => return Ok(project),
            Err(_) => return Err(()),
        };
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

        match DawProject::to_xml(project) {
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
        use quick_xml::se::to_string;

        let file = match std::fs::File::create(zip_file_path) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        let mut zip_writer = zip::ZipWriter::new(file);

        let project_xml = match to_string(&project) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        let meta_data_xml = match to_string(&meta_data) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        match Self::add_file_to_zip(&zip_writer, &project_xml, PROJECT_FILE) {
            Ok(()) => (),
            Err(_) => return Err(()),
        }

        match Self::add_file_to_zip(&zip_writer, &meta_data_xml, METADATA_FILE) {
            Ok(()) => (),
            Err(_) => return Err(()),
        }

        Ok(())
    }

    fn add_file_to_zip<W: std::io::Write + std::io::Seek>(
        zip_writer: &ZipWriter<W>,
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

        match zip_writer.write_all(text_content.as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err(()),
        }

        Ok(())
    }

    fn add_to_zip_file(zip_path: &Path, file_path: &Path) -> Result<(), ()> {
        let file = std::fs::File::create(zip_path).unwrap();

        let mut zip = zip::ZipWriter::new(file);
        /*xw
              final ZipEntry entry = new ZipEntry(path);
        zos.putNextEntry(entry);

        try (FileInputStream fileInputStream = new FileInputStream(file))
        {
            byte[] data = new byte[65536];
            int size = 0;
            while((size = fileInputStream.read(data)) != -1)
               zos.write(data, 0, size);

            zos.flush();
        }

        zos.closeEntry();
             */
        Ok(())
    }

    pub fn strip_bom() {
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

    pub fn load_project(fname: &Path) -> Result<(), ()> {
        let zip_file = std::fs::File::open(fname).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

        let mut file = match archive.by_name("fname/{}".format(PROJECT_FILE)) {
            Ok(file) => file,
            Err(_) => todo!(),
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{contents}");

        Ok(contents)
    }

    pub fn load_metadata(fname: &Path) -> Result<String, ()> {
        let zip_file = std::fs::File::open(fname).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

        let mut file = match archive.by_name("fname/{}".format(METADATA_FILE)) {
            Ok(file) => file,
            Err(_) => todo!(),
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{contents}");

        Ok(contents)
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
