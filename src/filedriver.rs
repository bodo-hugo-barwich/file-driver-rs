/*
# @author Bodo (Hugo) Barwich
# @version 2018-11-14
# @package fileaccess
# @subpackage struct FileDriver

# This Module defines Classes to manage the Access to Files in persistent or instant Mode
#
#---------------------------------
# Requirements:
*
*/



use std::str;
use std::io;
use std::io::Read;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;



/*
/// A Struct to operate on physical Files in a easy and convienent Way
*/
pub struct FileDriver {
  _file: Option<File>
  , _sdirectory: String
  , _sfile: String
  /*
  ///String of the Route to the File
  */
  , _spath: String
  , _arrcontent: Vec<Vec<u8>>
  , _vchunk: Vec<u8>
  , _scontent: Option<String>
  , _oschunk: Option<String>
  , _chunk_size: u16
  , _breadable: bool
  , _bwriteable: bool
  , _bappendable: bool
  , _bbuffered: bool
  , _bpersistent: bool
  , _bdebug: bool
  , _srpt: String
  , _serr: String
  , _ierr: i8
}


impl Default for FileDriver {
  /*----------------------------------------------------------------------------
   * Default Constructor
   */


    fn default() -> Self {
        FileDriver::new()
    }
}

impl FileDriver {
  /*----------------------------------------------------------------------------
   * Constructors
   */


  pub fn new() -> FileDriver {
    let mut drv = FileDriver { _file: None
      , _sdirectory: String::new()
      , _sfile: String::new(), _spath: String::new()
      , _arrcontent: Vec::new(), _vchunk: Vec::new()
      , _scontent: None, _oschunk: None
      , _chunk_size: 32768
      , _breadable: false, _bwriteable: false, _bappendable: false
      , _bbuffered: true, _bpersistent: false, _bdebug: false
      , _srpt: String::new(), _serr: String::new(), _ierr: 0 };

    drv._init();


    //Return the New FileDriver Object
    drv
  }

  pub fn from_directory_name(sdirectoryname: &str) -> FileDriver {
    let mut drv = FileDriver { _file: None
      , _sdirectory: String::from(sdirectoryname)
      , _sfile: String::new(), _spath: String::new()
      , _arrcontent: Vec::new(), _vchunk: Vec::new()
      , _scontent: None, _oschunk: None
      , _chunk_size: 32768
      , _breadable: false, _bwriteable: false, _bappendable: false
      , _bbuffered: true, _bpersistent: false, _bdebug: false
      , _srpt: String::new(), _serr: String::new(), _ierr: 0 };


    if ! drv._sdirectory.is_empty()
      && ! drv._sdirectory.ends_with('/') {
      drv._sdirectory.push('/');
    }

    drv._init();


    //Return the New FileDriver Object
    drv
  }

  pub fn from_file_name(sdirectoryname: &str, sfilename: &str) -> FileDriver {
    let mut drv = FileDriver { _file: None
      , _sdirectory: String::from(sdirectoryname)
      , _sfile: String::from(sfilename), _spath: String::new()
      , _arrcontent: Vec::new(), _vchunk: Vec::new()
      , _scontent: None, _oschunk: None
      , _chunk_size: 32768
      , _breadable: false, _bwriteable: false, _bappendable: false
      , _bbuffered: true, _bpersistent: false, _bdebug: false
      , _srpt: String::new(), _serr: String::new(), _ierr: 0 };


    if ! drv._sdirectory.is_empty()
      && ! drv._sdirectory.ends_with('/') {
      drv._sdirectory.push('/');
    }

    drv._init();


    //Return the New FileDriver Object
    drv
  }

  pub fn from_path_name(sfilepath: &str) -> FileDriver {
    let mut drv = FileDriver { _file: None
      , _sdirectory: String::new()
      , _sfile: String::new(), _spath: String::from(sfilepath)
      , _arrcontent: Vec::new(), _vchunk: Vec::new()
      , _scontent: None, _oschunk: None
      , _chunk_size: 32768
      , _breadable: false, _bwriteable: false, _bappendable: false
      , _bbuffered: true, _bpersistent: false, _bdebug: false
      , _srpt: String::new(), _serr: String::new(), _ierr: 0 };

    drv._init();


    //Return the New FileDriver Object
    drv
  }


  fn _init(&mut self) {
    if self._sfile.is_empty()
      && ! self._spath.is_empty() {
      match self._spath.rfind('/') {
        Some(ps) => {
          let (sdir, sfl) = self._spath.split_at(ps + 1);

          self._sdirectory = String::from(sdir);
          self._sfile = String::from(sfl);
          }
        , None => {
          self._sdirectory.truncate(0);
          self._sfile = String::from(self._spath.as_str());
          }
      } //match self._spath.rfind('/')
    } //if self._sfile.is_empty()! && self._spath.is_emtpy()

    if self._spath.is_empty()
      && ! self._sfile.is_empty() {
      //Build the File Path
      self._spath.push_str(self._sdirectory.as_str());
      self._spath.push_str(self._sfile.as_str());
    } //if self._spath.is_empty() && ! self._sfile.is_empty()
  }
}


impl Drop for FileDriver {
  /*----------------------------------------------------------------------------
   * Destructor
   */


  fn drop(&mut self) {
    //Close the File
    self.free_resources();
  }
}


impl FileDriver {
  /*
  #----------------------------------------------------------------------------
  #Administration Methods
  */

  pub fn set_directory_name(&mut self, sdirectoryname: &str) {
    self._sdirectory = String::from(sdirectoryname);

    if ! self._sdirectory.is_empty()
      && ! self._sdirectory.ends_with('/') {
      self._sdirectory.push('/');
    }

    //Free any Resources since the File is invalid
    self.free_resources();
  }

  pub fn set_file_name(&mut self, sfilename: &str) {
    self._sfile = String::from(sfilename);

    //Rebuild the File Path
    self._spath = String::new();
    self._spath.push_str(self._sdirectory.as_str());
    self._spath.push_str(self._sfile.as_str());

    //Free any Resources since the File is invalid
    self.free_resources();
  }

  pub fn set_path_name(&mut self, sfilepath: &str) {
    self._spath = String::from(sfilepath);

    self._sdirectory = String::new();
    self._sfile = String::new();

    match self._spath.as_str().rfind('/') {
      Some(ps) => {
          let (sdirnm, sflnm) = self._spath.as_str().split_at(ps + 1);


          self._sdirectory.push_str(sdirnm);
          self._sfile.push_str(sflnm);
        } //Some(ps)
      , None => self._sfile.push_str(self._spath.as_str())
    } //match self._file_path.as_str().rfind('/')

    //Free any Resources since the File is invalid
    self.free_resources();
  }


  pub fn set_chunk_size(&mut self, ichunksize: u16) {
    self._chunk_size = ichunksize;
  }


  pub fn set_buffered(&mut self, bbuffered: bool) {
    self._bbuffered = bbuffered;
  }


  pub fn set_persistent(&mut self, bpersistent: bool) {
    self._bpersistent = bpersistent;
  }


  pub fn set_debug(&mut self, bdebug: bool) {
    self._bdebug = bdebug;
  }


  fn _ropen(&mut self) -> bool {
    let mut brs = false;


    if ! self._is_open() {
      if self.exists() {
        match OpenOptions::new().read(true).open(&self._spath) {
          Ok(fl) => {
              self._file = Some(fl);
              self._breadable = true;

              if self._bdebug {
                self._srpt.push_str(&format!("File '{}': opened\n'{:?}'\n", self._spath
                  , self._file.as_ref()));
              }

          } //Ok(fl)
          , Err(e) => {
              self._serr.push_str(&format!("File '{}': Open Read failed!\n", &self._spath));
              self._serr.push_str(&format!("Message: {:?}", e));

              self._ierr = 1;
          }
        } //match OpenOptions::new().read(true).open(self._file_path)
      }
      else {
        self._serr.push_str(&format!("File '{}': Open Read failed!\n", &self._spath));
        self._serr.push_str(&"The File does not exist!\n");

        self._ierr = 3;
      } //if self.Exists()
    }
    else {
      brs = true;
    }


    brs
  }


  pub fn read_once(&mut self) -> bool {
    let mut brs = false;

    if ! self._is_readable() {
      self._close();
    }

    if ! self._is_open() {
      self._ropen();

      if self._bbuffered {
        //Drop old Content and create new empty one
        self._arrcontent = Vec::new();
        self._scontent = Some(String::new());
      }  //if self._bbuffered
    }  //if ! self._is_open()

    if self._is_readable() {
      let mut igtcnt = 0;


      match &mut self._file {
        Some(fl) => {
          let icksz = self._chunk_size.into();
          let mut vbff = vec![0; icksz];
          let mut vlstbts: Vec<u8> = Vec::new();


          self._oschunk = Some(String::new());

          if self._vchunk.len() < icksz {
              if self._bdebug {
                self._srpt.push_str(&format!("chunk do reserve 0 (sz: '{} / {}')\n"
                  , icksz - self._vchunk.len(), icksz));
              }

            self._vchunk.reserve(icksz - self._vchunk.len());
          }

              if self._bdebug {
                self._srpt.push_str(&format!("chunk 0 (sz: '{} / {} / {}')\n"
                  , self._vchunk.len() , self._vchunk.capacity(), icksz));
              }


          //Fill the Buffer from the File
          match fl.read(&mut vbff) {
            Ok(irs) => {
              igtcnt = irs;

              if self._bdebug {
                self._srpt.push_str(&format!("chunk (sz: '{} / {}'):\n'{:?}'\n", irs
                  , vbff.len(), vbff));
              }

              if igtcnt > 0 {
                vbff.truncate(igtcnt);

                self._vchunk.append(&mut vbff);


                //Try UTF8 Conversion
                let utf8rs = str::from_utf8(&self._vchunk);

                    match utf8rs {
                      Ok(s) => {
                        match &mut self._oschunk {
                          Some(sc) => { sc.push_str(&s); }
                          , None => {}
                        } //match &mut self._scontent

                      }
                      Err(e) => {
                        let mut after_valid = self._vchunk.split_off(e.valid_up_to());

                        match &mut self._oschunk {
                          Some(sc) => {
                            unsafe {
                                sc.push_str(str::from_utf8_unchecked(&self._vchunk));
                            }
                          }
                          , None => {}
                        }  //if let Some(mut sc) = self._oschunk

                        vlstbts.append(&mut after_valid);
                      }  //Err(e)
                    }  //match utf8rs

                    if self._bbuffered {
                    //Add the Buffer to the Content
                    self._arrcontent.push(self._vchunk.clone());
                    }


          self._vchunk.clear();

                //Add left over Bytes
                if ! vlstbts.is_empty() {

                self._vchunk.append(&mut vlstbts);

                }  //if ! vlstbts.is_empty()


              } //if igtcnt > 0



            }  //Ok(irs)
            , Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
              //Continue the Reading
              igtcnt = 1;
            }
            , Err(e) => {
              //Stop the Reading
              igtcnt = 0;

              self._serr.push_str(&format!("File '{}': Read failed!", self._spath));
              self._serr.push_str(&format!("Message: {:?}", e));

              self._ierr = 1;
            }  //Err(e)
          } //match fl.Read(&mut vcnk)

        }  //Some(fl)
        , None => {}
      } //match &mut self._file

      if self._ierr == 0
        && igtcnt > 0 {
        brs = true;
      }

      if ! self._bpersistent
        || self._ierr != 0
        || igtcnt == 0 {

        //Close the File
        self._close();

        //Communicate File End
        brs = false;
      }

    }  //if self._is_readable()

              if self._bdebug {
                self._srpt.push_str(&format!("read_once  finished with '{:?}'):\n''\n", brs));
              }

    //Return the Result
    brs
  }


  pub fn read(&mut self) -> bool {
    let mut brs = false;

    self._bbuffered = true;

    if ! self._is_readable() {
      self._close();
    }

    if ! self._is_open() {
      self._ropen();
    }

    if self._is_readable() {
      //Drop old Content and create new empty one
      self._arrcontent = Vec::new();
      self._scontent = Some(String::new());
      self._oschunk = Some(String::new());

      match &mut self._file {
        Some(fl) => {
          let mut vlstbts: Vec<u8> = Vec::new();
          let icksz = self._chunk_size.into();
          let mut ickrs = icksz;


          if self._bdebug {
            self._srpt.push_str(&format!("rd fl: '{:?}'\n", fl));
            self._srpt.push_str(&format!("cnk sz: '{}'\n", icksz));
          }

          while ickrs > 0
            && ickrs == icksz {
            if self._vchunk.len() < icksz {
              self._vchunk.reserve(icksz - self._vchunk.len());
            }

            //Fill the Buffer from the File
            match fl.read(&mut self._vchunk) {
              Ok(irs) => {
                  ickrs = irs;

                  if self._bdebug {
                    self._srpt.push_str(&format!("chunk (sz: '{}'):\n'{:?}'\n", irs, self._vchunk));
                  }

                  if ickrs > 0 {
                    self._vchunk.truncate(ickrs);

                    if ! vlstbts.is_empty() {
                      while let Some(bt) = vlstbts.pop() {
                        self._vchunk.insert(0, bt);
                      }
                    }  //if ! vlstbts.is_empty()

                    //Try UTF8 Conversion
                    let utf8rs = str::from_utf8(&self._vchunk);

                    match utf8rs {
                      Ok(s) => {
                        match &mut self._oschunk {
                          Some(sc) => { sc.push_str(&s); }
                          , None => {}
                        } //match &mut self._scontent

                      }
                      Err(e) => {
                        let mut after_valid = self._vchunk.split_off(e.valid_up_to());

                        match &mut self._oschunk {
                          Some(sc) => {
                            unsafe {
                                sc.push_str(str::from_utf8_unchecked(&self._vchunk));
                            }
                          }
                          , None => {}
                        }  //if let Some(mut sc) = self._oschunk

                        vlstbts.append(&mut after_valid);
                      }  //Err(e)
                    }  //match utf8rs

                    //Add the Buffer to the Content
                    self._arrcontent.push(self._vchunk.clone());
                  } //if ickrs > 0
              }  //Ok(irs)
              , Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                  //Continue the Reading
                  ickrs = 1;
              }
              , Err(e) => {
                  //Interrupt the Reading
                  ickrs = 0;

                  self._serr.push_str(&format!("File '{}': Read failed!", self._spath));
                  self._serr.push_str(&format!("Message: {:?}", e));

                  self._ierr = 1;
              }  //Err(e)
            } //match fl.read(&mut self._vchunk)
          } //while ickrs > 0 && ickrs == icksz

          if ! self._arrcontent.is_empty() {
            //Build the Content String
            let mut opbt = None;


            for mut vs in self._arrcontent.drain(..) {
              let mut bsok = false;


              if let Some(bt) = opbt {
                self._srpt.push_str(&format!("byte join: '{}'\n", bt));
                vs.insert(0, bt);
                opbt = None;
              } //if opbt.is_some()


              //Try UTF8 Conversion
              let rss = str::from_utf8(&vs);

              if let Ok(s) = rss  {
                match &mut self._scontent {
                  Some(sc) => { sc.push_str(&s); }
                  , None => {}
                } //match &mut self._scontent

                bsok = true;
              } //match rss

              if ! bsok {
                //Last Byte is not a complete Character
                opbt = vs.pop();

                self._srpt.push_str(&format!("byte pop: '{}'\n", opbt.unwrap()));
                self._srpt.push_str(&format!("chunk UTF8 retry:\n'{:?}'\n", vs));


                //Retry UTF8 Conversion
                let rss = str::from_utf8(&vs);


                match rss {
                  Ok(s) => {
                    match &mut self._scontent {
                      Some(sc) => { sc.push_str(&s); }
                      , None => {}
                    } //match &mut self._scontent
                    }
                  , Err(e) => {
                    self._serr.push_str(&format!("File '{}': Read failed!", self._spath));
                    self._serr.push_str(&"Message: Content is not valid UTF8 Text.");
                    self._serr.push_str(&format!("Info: {:?}", e));

                    self._ierr = 1;
                    }
                } //match rss
              } //if ! bsok
            } //for vs in self._arrcontent.drain(..)
          } //if if ! self._arrcontent.is_empty()

          if self._ierr == 0 {
            brs = true;
          }
        }  //Some(fl)
        , None => {}
      } //if let mut Some(fl) = self._file
    } //if self._isReadable()


    //Return the Result
    brs
  }


  fn _close(&mut self) -> bool {
    let mut brs = false;


    if self._is_open() {
      if let Some(fl) = &self._file {
          match fl.sync_all() {
            Ok(_) => {
              brs = true;
              }
            , Err(e) => {
              self._serr.push_str(&format!("File '{}': Write Disk failed!"
                , self._spath));
              self._serr.push_str(&format!("Message: {:?}", e));

              self._ierr = 1;
              } //Err(e)
          } //match fl.sync_all()

          //Unset the File
          self._file = None;
       } //if let Some(fl) = &self._file
    }
    else {
      brs = true;
    } //if self._isOpen()


    //Return the Result
    brs
  }

  pub fn free_resources(&mut self) {
    if self._is_open() {
      self._close();
    }
  }


  /*
  #----------------------------------------------------------------------------
  #Consultation Methods
  */


  pub fn get_directory_name(&self) -> &str {
    self._sdirectory.as_str()
  }

  pub fn get_file_name(&self) -> &str {
    self._sfile.as_str()
  }

  pub fn get_path_name(&self) -> &str {
    self._spath.as_str()
  }


  pub fn exists(&self) -> bool {
    if ! self._is_open() {
      Path::new(self._spath.as_str()).exists()
    }
    else {
      true
    } //if self._is_open()
  }

  pub fn get_content(&self) -> &str {
    match &self._scontent {
      Some(cs) => { cs.as_str() }
      , None => { &"" }
    }   //match mut self._scontent
  }

  pub fn get_chunk(&self) -> &str {
    match &self._oschunk {
      Some(cs) => { cs.as_str() }
      , None => { &"" }
    }   //match mut self._oschunk
  }

  pub fn take_content(&mut self) -> String {
    if self._scontent.is_some() {
      self._scontent.take().unwrap()
    } else {
      self._serr.push_str(&format!("File '{}': Content Access failed.\n"
          , self._spath));
      self._serr.push_str(&"Message: Content is None.\n");

      String::new()
    }   //match mut self._scontent
  }

  fn _is_open(&self) -> bool {
    self._file.is_some()
  }

  fn _is_readable(&self) -> bool {
    if self._file.is_some() {
      self._breadable
    }
    else {
      false
    }
  }

  pub fn get_report_string(&self) -> &str {
    self._srpt.as_str()
  }

  pub fn get_error_string(&self) -> &str {
    self._serr.as_str()
  }

  pub fn get_error_code(&self) -> i8 {
    self._ierr
  }
}