use std::error::Error;

use crate::meta_data::MetaData;

const FORMAT_NAME: &str = "DAWProject exchange format";
const FILE_EXTENSION: &str = "dawproject";
const PROJECT_FILE: &str = "project.xml";
const METADATA_FILE: &str = "metadata.xml";

struct DawProject {
    format_name: &str,
    file_extension: &str,
    project_file: &str,
    metadata_file: &str,
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

    pub fn export_schema() -> Result<Ok, Error()> {
        /*
        This probably be able to export xml schema.xsd, but it is unnecessary since we will always derive it
        from its parent project in Java.
         */
        Ok(())
    }

    pub fn to_xml(project: Project) -> Result<ok, Error()> {
        /*
        I think it just takes project and saves it as xml file
        So we can pass for example cloned object, generate String from it
        and save it as xml file.
         */

        use quick_xml::se::to_string;

        let mut xml_string: String;
        match to_string(Project) {
            Ok(object) => xml_string = object,
            Err(_) => return Error(()),
        };

        Ok(())
    }

    pub fn create_context() -> Result<Ok(), Error(())> {
        /*
        This functions is creating some JABCONTEXT, whatever it is, unnecessary I think
         */
        Ok(())
    }

    pub fn from_xml(xml_string: String) -> Result<Ok(Project), Error(())> {
        use quick_xml::de::from_str;

        match from_str(&xml_string) {
            Ok(project) => return Ok(project),
            Err(_) => return Error(()),
        };
    }

    pub fn save_xml() -> Result<Ok(()), Error(())> {
        /*
          This is purely for file save handling

          String projectXML = toXML(project);
        FileOutputStream fileOutputStream = new FileOutputStream(file);
        fileOutputStream.write(projectXML.getBytes(StandardCharsets.UTF_8));
        fileOutputStream.close();
           */

        Ok(())
    }

    pub fn validate(project: Project) -> Result<Ok(()), Error(())> {
        /*
        Should be some sort of validation mechanism which checks whether
        xml-ed project struct work accordingly to schema
         */

        Ok(())
    }

    pub fn save(
        project: Project,
        meta_data: MetaData,
        embedded_files: HashMap<File, String>,
        file: File,
    ) -> Result<Ok(), Error> {
        use quick_xml::se::to_string;

        let meta_data_xml;
        let project_xml;

        match to_string(project) {
            Ok(s) => {
                project_xml = s;
            }
            Err(_) => return Err(()),
        };

        match to_string(&meta_data) {
            Ok(s) => {
                meta_data_xml = s;
            }
            Err(_) => return Err(()),
        };
        Ok(())

        /*
        Here zipping both files and embedded files continues so they are ziipped into one file.
         */
    }

    fn add_to_zip_bytes(zos: ZipOutputStream, path: String, data: [u8]) {
        /*
           final ZipEntry entry = new ZipEntry(path);
        zos.putNextEntry(entry);
        zos.write(data);
        zos.closeEntry();
           */
    }

    fn add_to_zip_file(zos: ZipOutputStream, path: String, file: File) {
        /*
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

    pub fn load_project(file: File) -> Result<Ok(()), Error(())> {
        // Sequential call of above functions

        Ok(())
    }

    pub fn load_metadata(file: File) -> Result<Ok(()), Error(())> {
        // Sequential call of above functions

        Ok(())
    }

    pub fn stream_embedded(file: File, embedded_path: String) -> Result<Ok(()), Error(())> {
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
    }
}
