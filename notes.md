1.Create required constructors for all of structs to make it easier  and without a bugs to  create the objects 
2. Make fields required in tests public so one can access them when creating
3. Attributes are optional by default 
4. When a top level class or an enum type is annotated with the @XmlRootElement annotation, then its value is represented as XML element in an XML document. 
5. When we refer to some field by IDREF we do not store full type but its name 
6. The presence of @XmlElementRef indicates that the XML. element name will be derived from the @XmlRootElement . annotation on the type (for e.g. "jar" for JarTask).
7. In an XML document or external entity, a CDATA section is a piece of element content that is marked up to be interpreted literally, as textual data, not as marked-up content.
8. I think idref is just a string so there is not necessarily a lot of to do in that case,  Interpreting that internally depends on us