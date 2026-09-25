use super::*;

/// Enumeration of supported file extensions.
///
/// Contains all known file extensions with their corresponding MIME types.
/// Used for content type detection and file handling.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum FileExtension {
    /// The `123` file extension.
    FileExtension123,
    /// The `3dml` file extension.
    FileExtension3dml,
    /// The `3ds` file extension.
    FileExtension3ds,
    /// The `3g2` file extension.
    FileExtension3g2,
    /// The `3gp` file extension.
    FileExtension3gp,
    /// The `7z` file extension.
    FileExtension7z,
    /// The `aab` file extension.
    FileExtensionAab,
    /// The `aac` file extension.
    FileExtensionAac,
    /// The `aam` file extension.
    FileExtensionAam,
    /// The `aas` file extension.
    FileExtensionAas,
    /// The `abs` file extension.
    FileExtensionAbs,
    /// The `abw` file extension.
    FileExtensionAbw,
    /// The `ac` file extension.
    FileExtensionAc,
    /// The `acc` file extension.
    FileExtensionAcc,
    /// The `ace` file extension.
    FileExtensionAce,
    /// The `acu` file extension.
    FileExtensionAcu,
    /// The `acutc` file extension.
    FileExtensionAcutc,
    /// The `adp` file extension.
    FileExtensionAdp,
    /// The `aep` file extension.
    FileExtensionAep,
    /// The `afm` file extension.
    FileExtensionAfm,
    /// The `afp` file extension.
    FileExtensionAfp,
    /// The `ahead` file extension.
    FileExtensionAhead,
    /// The `ai` file extension.
    FileExtensionAi,
    /// The `aif` file extension.
    FileExtensionAif,
    /// The `aifc` file extension.
    FileExtensionAifc,
    /// The `aiff` file extension.
    FileExtensionAiff,
    /// The `aim` file extension.
    FileExtensionAim,
    /// The `air` file extension.
    FileExtensionAir,
    /// The `ait` file extension.
    FileExtensionAit,
    /// The `ami` file extension.
    FileExtensionAmi,
    /// The `anx` file extension.
    FileExtensionAnx,
    /// The `apk` file extension.
    FileExtensionApk,
    /// The `appcache` file extension.
    FileExtensionAppcache,
    /// The `application` file extension.
    FileExtensionApplication,
    /// The `apr` file extension.
    FileExtensionApr,
    /// The `arc` file extension.
    FileExtensionArc,
    /// The `art` file extension.
    FileExtensionArt,
    /// The `asc` file extension.
    FileExtensionAsc,
    /// The `asf` file extension.
    FileExtensionAsf,
    /// The `asm` file extension.
    FileExtensionAsm,
    /// The `aso` file extension.
    FileExtensionAso,
    /// The `asx` file extension.
    FileExtensionAsx,
    /// The `atc` file extension.
    FileExtensionAtc,
    /// The `atom` file extension.
    FileExtensionAtom,
    /// The `atomcat` file extension.
    FileExtensionAtomcat,
    /// The `atomsvc` file extension.
    FileExtensionAtomsvc,
    /// The `atx` file extension.
    FileExtensionAtx,
    /// The `au` file extension.
    FileExtensionAu,
    /// The `avi` file extension.
    FileExtensionAvi,
    /// The `avx` file extension.
    FileExtensionAvx,
    /// The `aw` file extension.
    FileExtensionAw,
    /// The `axa` file extension.
    FileExtensionAxa,
    /// The `axv` file extension.
    FileExtensionAxv,
    /// The `azf` file extension.
    FileExtensionAzf,
    /// The `azs` file extension.
    FileExtensionAzs,
    /// The `azw` file extension.
    FileExtensionAzw,
    /// The `bat` file extension.
    FileExtensionBat,
    /// The `bcpio` file extension.
    FileExtensionBcpio,
    /// The `bdf` file extension.
    FileExtensionBdf,
    /// The `bdm` file extension.
    FileExtensionBdm,
    /// The `bed` file extension.
    FileExtensionBed,
    /// The `bh2` file extension.
    FileExtensionBh2,
    /// The `bin` file extension.
    FileExtensionBin,
    /// The `blb` file extension.
    FileExtensionBlb,
    /// The `blorb` file extension.
    FileExtensionBlorb,
    /// The `bmi` file extension.
    FileExtensionBmi,
    /// The `bmp` file extension.
    FileExtensionBmp,
    /// The `body` file extension.
    FileExtensionBody,
    /// The `book` file extension.
    FileExtensionBook,
    /// The `box` file extension.
    FileExtensionBox,
    /// The `boz` file extension.
    FileExtensionBoz,
    /// The `bpk` file extension.
    FileExtensionBpk,
    /// The `btif` file extension.
    FileExtensionBtif,
    /// The `bz` file extension.
    FileExtensionBz,
    /// The `bz2` file extension.
    FileExtensionBz2,
    /// The `c` file extension.
    FileExtensionC,
    /// The `c11amc` file extension.
    FileExtensionC11amc,
    /// The `c11amz` file extension.
    FileExtensionC11amz,
    /// The `c4d` file extension.
    FileExtensionC4d,
    /// The `c4f` file extension.
    FileExtensionC4f,
    /// The `c4g` file extension.
    FileExtensionC4g,
    /// The `c4p` file extension.
    FileExtensionC4p,
    /// The `c4u` file extension.
    FileExtensionC4u,
    /// The `cab` file extension.
    FileExtensionCab,
    /// The `caf` file extension.
    FileExtensionCaf,
    /// The `cap` file extension.
    FileExtensionCap,
    /// The `car` file extension.
    FileExtensionCar,
    /// The `cat` file extension.
    FileExtensionCat,
    /// The `cb7` file extension.
    FileExtensionCb7,
    /// The `cba` file extension.
    FileExtensionCba,
    /// The `cbr` file extension.
    FileExtensionCbr,
    /// The `cbt` file extension.
    FileExtensionCbt,
    /// The `cbz` file extension.
    FileExtensionCbz,
    /// The `cc` file extension.
    FileExtensionCc,
    /// The `cct` file extension.
    FileExtensionCct,
    /// The `ccxml` file extension.
    FileExtensionCcxml,
    /// The `cdbcmsg` file extension.
    FileExtensionCdbcmsg,
    /// The `cdf` file extension.
    FileExtensionCdf,
    /// The `cdkey` file extension.
    FileExtensionCdkey,
    /// The `cdmia` file extension.
    FileExtensionCdmia,
    /// The `cdmic` file extension.
    FileExtensionCdmic,
    /// The `cdmid` file extension.
    FileExtensionCdmid,
    /// The `cdmio` file extension.
    FileExtensionCdmio,
    /// The `cdmiq` file extension.
    FileExtensionCdmiq,
    /// The `cdx` file extension.
    FileExtensionCdx,
    /// The `cdxml` file extension.
    FileExtensionCdxml,
    /// The `cdy` file extension.
    FileExtensionCdy,
    /// The `cer` file extension.
    FileExtensionCer,
    /// The `cfs` file extension.
    FileExtensionCfs,
    /// The `cgm` file extension.
    FileExtensionCgm,
    /// The `chat` file extension.
    FileExtensionChat,
    /// The `chm` file extension.
    FileExtensionChm,
    /// The `chrt` file extension.
    FileExtensionChrt,
    /// The `cif` file extension.
    FileExtensionCif,
    /// The `cii` file extension.
    FileExtensionCii,
    /// The `cil` file extension.
    FileExtensionCil,
    /// The `cla` file extension.
    FileExtensionCla,
    /// The `class` file extension.
    FileExtensionClass,
    /// The `clkk` file extension.
    FileExtensionClkk,
    /// The `clkp` file extension.
    FileExtensionClkp,
    /// The `clkt` file extension.
    FileExtensionClkt,
    /// The `clkw` file extension.
    FileExtensionClkw,
    /// The `clkx` file extension.
    FileExtensionClkx,
    /// The `clp` file extension.
    FileExtensionClp,
    /// The `cmc` file extension.
    FileExtensionCmc,
    /// The `cmdf` file extension.
    FileExtensionCmdf,
    /// The `cml` file extension.
    FileExtensionCml,
    /// The `cmp` file extension.
    FileExtensionCmp,
    /// The `cmx` file extension.
    FileExtensionCmx,
    /// The `cod` file extension.
    FileExtensionCod,
    /// The `com` file extension.
    FileExtensionCom,
    /// The `conf` file extension.
    FileExtensionConf,
    /// The `cpio` file extension.
    FileExtensionCpio,
    /// The `cpp` file extension.
    FileExtensionCpp,
    /// The `cpt` file extension.
    FileExtensionCpt,
    /// The `crd` file extension.
    FileExtensionCrd,
    /// The `crl` file extension.
    FileExtensionCrl,
    /// The `crt` file extension.
    FileExtensionCrt,
    /// The `cryptonote` file extension.
    FileExtensionCryptonote,
    /// The `csh` file extension.
    FileExtensionCsh,
    /// The `csml` file extension.
    FileExtensionCsml,
    /// The `csp` file extension.
    FileExtensionCsp,
    /// The `css` file extension.
    FileExtensionCss,
    /// The `cst` file extension.
    FileExtensionCst,
    /// The `csv` file extension.
    FileExtensionCsv,
    /// The `cu` file extension.
    FileExtensionCu,
    /// The `curl` file extension.
    FileExtensionCurl,
    /// The `cww` file extension.
    FileExtensionCww,
    /// The `cxt` file extension.
    FileExtensionCxt,
    /// The `cxx` file extension.
    FileExtensionCxx,
    /// The `dae` file extension.
    FileExtensionDae,
    /// The `daf` file extension.
    FileExtensionDaf,
    /// The `dart` file extension.
    FileExtensionDart,
    /// The `dataless` file extension.
    FileExtensionDataless,
    /// The `davmount` file extension.
    FileExtensionDavmount,
    /// The `dbk` file extension.
    FileExtensionDbk,
    /// The `dcr` file extension.
    FileExtensionDcr,
    /// The `dcurl` file extension.
    FileExtensionDcurl,
    /// The `dd2` file extension.
    FileExtensionDd2,
    /// The `ddd` file extension.
    FileExtensionDdd,
    /// The `deb` file extension.
    FileExtensionDeb,
    /// The `def` file extension.
    FileExtensionDef,
    /// The `deploy` file extension.
    FileExtensionDeploy,
    /// The `der` file extension.
    FileExtensionDer,
    /// The `dfac` file extension.
    FileExtensionDfac,
    /// The `dgc` file extension.
    FileExtensionDgc,
    /// The `dib` file extension.
    FileExtensionDib,
    /// The `dic` file extension.
    FileExtensionDic,
    /// The `dir` file extension.
    FileExtensionDir,
    /// The `dis` file extension.
    FileExtensionDis,
    /// The `dist` file extension.
    FileExtensionDist,
    /// The `distz` file extension.
    FileExtensionDistz,
    /// The `djv` file extension.
    FileExtensionDjv,
    /// The `djvu` file extension.
    FileExtensionDjvu,
    /// The `dll` file extension.
    FileExtensionDll,
    /// The `dmg` file extension.
    FileExtensionDmg,
    /// The `dmp` file extension.
    FileExtensionDmp,
    /// The `dms` file extension.
    FileExtensionDms,
    /// The `dna` file extension.
    FileExtensionDna,
    /// The `doc` file extension.
    FileExtensionDoc,
    /// The `docm` file extension.
    FileExtensionDocm,
    /// The `docx` file extension.
    FileExtensionDocx,
    /// The `dot` file extension.
    FileExtensionDot,
    /// The `dotm` file extension.
    FileExtensionDotm,
    /// The `dotx` file extension.
    FileExtensionDotx,
    /// The `dp` file extension.
    FileExtensionDp,
    /// The `dpg` file extension.
    FileExtensionDpg,
    /// The `dra` file extension.
    FileExtensionDra,
    /// The `dsc` file extension.
    FileExtensionDsc,
    /// The `dssc` file extension.
    FileExtensionDssc,
    /// The `dtb` file extension.
    FileExtensionDtb,
    /// The `dtd` file extension.
    FileExtensionDtd,
    /// The `dts` file extension.
    FileExtensionDts,
    /// The `dtshd` file extension.
    FileExtensionDtshd,
    /// The `dump` file extension.
    FileExtensionDump,
    /// The `dv` file extension.
    FileExtensionDv,
    /// The `dvb` file extension.
    FileExtensionDvb,
    /// The `dvi` file extension.
    FileExtensionDvi,
    /// The `dwf` file extension.
    FileExtensionDwf,
    /// The `dwg` file extension.
    FileExtensionDwg,
    /// The `dxf` file extension.
    FileExtensionDxf,
    /// The `dxp` file extension.
    FileExtensionDxp,
    /// The `dxr` file extension.
    FileExtensionDxr,
    /// The `ecelp4800` file extension.
    FileExtensionEcelp4800,
    /// The `ecelp7470` file extension.
    FileExtensionEcelp7470,
    /// The `ecelp9600` file extension.
    FileExtensionEcelp9600,
    /// The `ecma` file extension.
    FileExtensionEcma,
    /// The `edm` file extension.
    FileExtensionEdm,
    /// The `edx` file extension.
    FileExtensionEdx,
    /// The `efif` file extension.
    FileExtensionEfif,
    /// The `ei6` file extension.
    FileExtensionEi6,
    /// The `elc` file extension.
    FileExtensionElc,
    /// The `emf` file extension.
    FileExtensionEmf,
    /// The `eml` file extension.
    FileExtensionEml,
    /// The `emma` file extension.
    FileExtensionEmma,
    /// The `emz` file extension.
    FileExtensionEmz,
    /// The `eol` file extension.
    FileExtensionEol,
    /// The `eot` file extension.
    FileExtensionEot,
    /// The `eps` file extension.
    FileExtensionEps,
    /// The `epub` file extension.
    FileExtensionEpub,
    /// The `es3` file extension.
    FileExtensionEs3,
    /// The `esa` file extension.
    FileExtensionEsa,
    /// The `esf` file extension.
    FileExtensionEsf,
    /// The `et3` file extension.
    FileExtensionEt3,
    /// The `etx` file extension.
    FileExtensionEtx,
    /// The `eva` file extension.
    FileExtensionEva,
    /// The `evy` file extension.
    FileExtensionEvy,
    /// The `exe` file extension.
    FileExtensionExe,
    /// The `exi` file extension.
    FileExtensionExi,
    /// The `ext` file extension.
    FileExtensionExt,
    /// The `ez` file extension.
    FileExtensionEz,
    /// The `ez2` file extension.
    FileExtensionEz2,
    /// The `ez3` file extension.
    FileExtensionEz3,
    /// The `f` file extension.
    FileExtensionF,
    /// The `f4v` file extension.
    FileExtensionF4v,
    /// The `f77` file extension.
    FileExtensionF77,
    /// The `f90` file extension.
    FileExtensionF90,
    /// The `fbs` file extension.
    FileExtensionFbs,
    /// The `fcdt` file extension.
    FileExtensionFcdt,
    /// The `fcs` file extension.
    FileExtensionFcs,
    /// The `fdf` file extension.
    FileExtensionFdf,
    /// The `fe_launch` file extension.
    FileExtensionFeLaunch,
    /// The `fg5` file extension.
    FileExtensionFg5,
    /// The `fgd` file extension.
    FileExtensionFgd,
    /// The `fh` file extension.
    FileExtensionFh,
    /// The `fh4` file extension.
    FileExtensionFh4,
    /// The `fh5` file extension.
    FileExtensionFh5,
    /// The `fh7` file extension.
    FileExtensionFh7,
    /// The `fhc` file extension.
    FileExtensionFhc,
    /// The `fig` file extension.
    FileExtensionFig,
    /// The `flac` file extension.
    FileExtensionFlac,
    /// The `fli` file extension.
    FileExtensionFli,
    /// The `flo` file extension.
    FileExtensionFlo,
    /// The `flv` file extension.
    FileExtensionFlv,
    /// The `flw` file extension.
    FileExtensionFlw,
    /// The `flx` file extension.
    FileExtensionFlx,
    /// The `fly` file extension.
    FileExtensionFly,
    /// The `fm` file extension.
    FileExtensionFm,
    /// The `fnc` file extension.
    FileExtensionFnc,
    /// The `for` file extension.
    FileExtensionFor,
    /// The `fpx` file extension.
    FileExtensionFpx,
    /// The `frame` file extension.
    FileExtensionFrame,
    /// The `fsc` file extension.
    FileExtensionFsc,
    /// The `fst` file extension.
    FileExtensionFst,
    /// The `ftc` file extension.
    FileExtensionFtc,
    /// The `fti` file extension.
    FileExtensionFti,
    /// The `fvt` file extension.
    FileExtensionFvt,
    /// The `fxp` file extension.
    FileExtensionFxp,
    /// The `fxpl` file extension.
    FileExtensionFxpl,
    /// The `fzs` file extension.
    FileExtensionFzs,
    /// The `g2w` file extension.
    FileExtensionG2w,
    /// The `g3` file extension.
    FileExtensionG3,
    /// The `g3w` file extension.
    FileExtensionG3w,
    /// The `gac` file extension.
    FileExtensionGac,
    /// The `gam` file extension.
    FileExtensionGam,
    /// The `gbr` file extension.
    FileExtensionGbr,
    /// The `gca` file extension.
    FileExtensionGca,
    /// The `gdl` file extension.
    FileExtensionGdl,
    /// The `geo` file extension.
    FileExtensionGeo,
    /// The `gex` file extension.
    FileExtensionGex,
    /// The `ggb` file extension.
    FileExtensionGgb,
    /// The `ggt` file extension.
    FileExtensionGgt,
    /// The `ghf` file extension.
    FileExtensionGhf,
    /// The `gif` file extension.
    FileExtensionGif,
    /// The `gim` file extension.
    FileExtensionGim,
    /// The `gml` file extension.
    FileExtensionGml,
    /// The `gmx` file extension.
    FileExtensionGmx,
    /// The `gnumeric` file extension.
    FileExtensionGnumeric,
    /// The `gph` file extension.
    FileExtensionGph,
    /// The `gpx` file extension.
    FileExtensionGpx,
    /// The `gqf` file extension.
    FileExtensionGqf,
    /// The `gqs` file extension.
    FileExtensionGqs,
    /// The `gram` file extension.
    FileExtensionGram,
    /// The `gramps` file extension.
    FileExtensionGramps,
    /// The `gre` file extension.
    FileExtensionGre,
    /// The `grv` file extension.
    FileExtensionGrv,
    /// The `grxml` file extension.
    FileExtensionGrxml,
    /// The `gsf` file extension.
    FileExtensionGsf,
    /// The `gtar` file extension.
    FileExtensionGtar,
    /// The `gtm` file extension.
    FileExtensionGtm,
    /// The `gtw` file extension.
    FileExtensionGtw,
    /// The `gv` file extension.
    FileExtensionGv,
    /// The `gxf` file extension.
    FileExtensionGxf,
    /// The `gxt` file extension.
    FileExtensionGxt,
    /// The `gz` file extension.
    FileExtensionGz,
    /// The `h` file extension.
    FileExtensionH,
    /// The `h261` file extension.
    FileExtensionH261,
    /// The `h263` file extension.
    FileExtensionH263,
    /// The `h264` file extension.
    FileExtensionH264,
    /// The `hal` file extension.
    FileExtensionHal,
    /// The `hbci` file extension.
    FileExtensionHbci,
    /// The `hdf` file extension.
    FileExtensionHdf,
    /// The `hh` file extension.
    FileExtensionHh,
    /// The `hlp` file extension.
    FileExtensionHlp,
    /// The `hpgl` file extension.
    FileExtensionHpgl,
    /// The `hpid` file extension.
    FileExtensionHpid,
    /// The `hps` file extension.
    FileExtensionHps,
    /// The `hqx` file extension.
    FileExtensionHqx,
    /// The `htc` file extension.
    FileExtensionHtc,
    /// The `htke` file extension.
    FileExtensionHtke,
    /// The `htm` file extension.
    FileExtensionHtm,
    /// The `html` file extension.
    FileExtensionHtml,
    /// The `hvd` file extension.
    FileExtensionHvd,
    /// The `hvp` file extension.
    FileExtensionHvp,
    /// The `hvs` file extension.
    FileExtensionHvs,
    /// The `i2g` file extension.
    FileExtensionI2g,
    /// The `icc` file extension.
    FileExtensionIcc,
    /// The `ice` file extension.
    FileExtensionIce,
    /// The `icm` file extension.
    FileExtensionIcm,
    /// The `ico` file extension.
    FileExtensionIco,
    /// The `ics` file extension.
    FileExtensionIcs,
    /// The `ief` file extension.
    FileExtensionIef,
    /// The `ifb` file extension.
    FileExtensionIfb,
    /// The `ifm` file extension.
    FileExtensionIfm,
    /// The `iges` file extension.
    FileExtensionIges,
    /// The `igl` file extension.
    FileExtensionIgl,
    /// The `igm` file extension.
    FileExtensionIgm,
    /// The `igs` file extension.
    FileExtensionIgs,
    /// The `igx` file extension.
    FileExtensionIgx,
    /// The `iif` file extension.
    FileExtensionIif,
    /// The `imp` file extension.
    FileExtensionImp,
    /// The `ims` file extension.
    FileExtensionIms,
    /// The `in` file extension.
    FileExtensionIn,
    /// The `ink` file extension.
    FileExtensionInk,
    /// The `inkml` file extension.
    FileExtensionInkml,
    /// The `install` file extension.
    FileExtensionInstall,
    /// The `iota` file extension.
    FileExtensionIota,
    /// The `ipfix` file extension.
    FileExtensionIpfix,
    /// The `ipk` file extension.
    FileExtensionIpk,
    /// The `irm` file extension.
    FileExtensionIrm,
    /// The `irp` file extension.
    FileExtensionIrp,
    /// The `iso` file extension.
    FileExtensionIso,
    /// The `itp` file extension.
    FileExtensionItp,
    /// The `ivp` file extension.
    FileExtensionIvp,
    /// The `ivu` file extension.
    FileExtensionIvu,
    /// The `jad` file extension.
    FileExtensionJad,
    /// The `jam` file extension.
    FileExtensionJam,
    /// The `jar` file extension.
    FileExtensionJar,
    /// The `java` file extension.
    FileExtensionJava,
    /// The `jisp` file extension.
    FileExtensionJisp,
    /// The `jlt` file extension.
    FileExtensionJlt,
    /// The `jnlp` file extension.
    FileExtensionJnlp,
    /// The `joda` file extension.
    FileExtensionJoda,
    /// The `jpe` file extension.
    FileExtensionJpe,
    /// The `jpeg` file extension.
    FileExtensionJpeg,
    /// The `jpg` file extension.
    FileExtensionJpg,
    /// The `jpgm` file extension.
    FileExtensionJpgm,
    /// The `jpgv` file extension.
    FileExtensionJpgv,
    /// The `jpm` file extension.
    FileExtensionJpm,
    /// The `js` file extension.
    FileExtensionJs,
    /// The `jsf` file extension.
    FileExtensionJsf,
    /// The `json` file extension.
    FileExtensionJson,
    /// The `jsonml` file extension.
    FileExtensionJsonml,
    /// The `jspf` file extension.
    FileExtensionJspf,
    /// The `kar` file extension.
    FileExtensionKar,
    /// The `karbon` file extension.
    FileExtensionKarbon,
    /// The `kfo` file extension.
    FileExtensionKfo,
    /// The `kia` file extension.
    FileExtensionKia,
    /// The `kml` file extension.
    FileExtensionKml,
    /// The `kmz` file extension.
    FileExtensionKmz,
    /// The `kne` file extension.
    FileExtensionKne,
    /// The `knp` file extension.
    FileExtensionKnp,
    /// The `kon` file extension.
    FileExtensionKon,
    /// The `kpr` file extension.
    FileExtensionKpr,
    /// The `kpt` file extension.
    FileExtensionKpt,
    /// The `kpxx` file extension.
    FileExtensionKpxx,
    /// The `ksp` file extension.
    FileExtensionKsp,
    /// The `ktr` file extension.
    FileExtensionKtr,
    /// The `ktx` file extension.
    FileExtensionKtx,
    /// The `ktz` file extension.
    FileExtensionKtz,
    /// The `kwd` file extension.
    FileExtensionKwd,
    /// The `kwt` file extension.
    FileExtensionKwt,
    /// The `lasxml` file extension.
    FileExtensionLasxml,
    /// The `latex` file extension.
    FileExtensionLatex,
    /// The `lbd` file extension.
    FileExtensionLbd,
    /// The `lbe` file extension.
    FileExtensionLbe,
    /// The `les` file extension.
    FileExtensionLes,
    /// The `lha` file extension.
    FileExtensionLha,
    /// The `link66` file extension.
    FileExtensionLink66,
    /// The `list` file extension.
    FileExtensionList,
    /// The `list3820` file extension.
    FileExtensionList3820,
    /// The `listafp` file extension.
    FileExtensionListafp,
    /// The `lnk` file extension.
    FileExtensionLnk,
    /// The `log` file extension.
    FileExtensionLog,
    /// The `lostxml` file extension.
    FileExtensionLostxml,
    /// The `lrf` file extension.
    FileExtensionLrf,
    /// The `lrm` file extension.
    FileExtensionLrm,
    /// The `ltf` file extension.
    FileExtensionLtf,
    /// The `lvp` file extension.
    FileExtensionLvp,
    /// The `lwp` file extension.
    FileExtensionLwp,
    /// The `lzh` file extension.
    FileExtensionLzh,
    /// The `m13` file extension.
    FileExtensionM13,
    /// The `m14` file extension.
    FileExtensionM14,
    /// The `m1v` file extension.
    FileExtensionM1v,
    /// The `m21` file extension.
    FileExtensionM21,
    /// The `m2a` file extension.
    FileExtensionM2a,
    /// The `m2v` file extension.
    FileExtensionM2v,
    /// The `m3a` file extension.
    FileExtensionM3a,
    /// The `m3u` file extension.
    FileExtensionM3u,
    /// The `m3u8` file extension.
    FileExtensionM3u8,
    /// The `m4a` file extension.
    FileExtensionM4a,
    /// The `m4b` file extension.
    FileExtensionM4b,
    /// The `m4r` file extension.
    FileExtensionM4r,
    /// The `m4u` file extension.
    FileExtensionM4u,
    /// The `m4v` file extension.
    FileExtensionM4v,
    /// The file extension for markdown.
    FileExtensionMarkdown,
    /// The file extension for toml.
    FileExtensionToml,
    /// The file extension for yaml.
    FileExtensionYaml,
    /// The file extension for yaml.
    FileExtensionYml,
    /// The file extension for ini.
    FileExtensionIni,
    /// The file extension for cfg.
    FileExtensionCfg,
    /// The file extension for Python.
    FileExtensionPython,
    /// The file extension for Go.
    FileExtensionGo,
    /// The file extension for TypeScript.
    FileExtensionTypeScript,
    /// The file extension for C#.
    FileExtensionCSharp,
    /// The file extension for PHP.
    FileExtensionPhp,
    /// The file extension for Ruby.
    FileExtensionRuby,
    /// The file extension for Swift.
    FileExtensionSwift,
    /// The file extension for Kotlin.
    FileExtensionKotlin,
    /// The file extension for Kotlin Script.
    FileExtensionKotlinScript,
    /// The file extension for Scala.
    FileExtensionScala,
    /// The file extension for IBM Secure Container or Scala Script.
    FileExtensionIbmScOrScalaScript,
    /// The file extension for Perl.
    FileExtensionPerl,
    /// The file extension for Perl Module.
    FileExtensionPerlModule,
    /// The file extension for Lua.
    FileExtensionLua,
    /// The file extension for PowerShell.
    FileExtensionPowerShell,
    /// The file extension for C++ Header.
    FileExtensionCppHeader,
    /// The file extension for Objective-C.
    FileExtensionObjectiveC,
    /// The file extension for Objective-C++.
    FileExtensionObjectiveCpp,
    /// The file extension for Groovy.
    FileExtensionGroovy,
    /// The file extension for R.
    FileExtensionR,
    /// The file extension for SCSS.
    FileExtensionScss,
    /// The file extension for SASS.
    FileExtensionSass,
    /// The file extension for LESS.
    FileExtensionLess,
    /// The file extension for Vue.
    FileExtensionVue,
    /// The file extension for JSX.
    FileExtensionJsx,
    /// The file extension for TSX.
    FileExtensionTsx,
    /// The file extension for Dockerfile.
    FileExtensionDockerfile,
    /// The file extension for Makefile.
    FileExtensionMakefile,
    /// The file extension for RLS Services XML or Rust.
    FileExtensionRs,
    /// The file extension for Haskell.
    FileExtensionHaskell,
    /// The file extension for Erlang.
    FileExtensionErlang,
    /// The file extension for Elixir.
    FileExtensionElixir,
    /// The file extension for Elixir Script.
    FileExtensionElixirScript,
    /// The file extension for Clojure.
    FileExtensionClojure,
    /// The file extension for ClojureScript.
    FileExtensionClojureScript,
    /// The file extension for Clojure Common.
    FileExtensionClojureCommon,
    /// The file extension for F#.
    FileExtensionFSharp,
    /// The file extension for F# Script.
    FileExtensionFSharpScript,
    /// The file extension for OCaml.
    FileExtensionOCaml,
    /// The file extension for OCaml Interface.
    FileExtensionOCamlInterface,
    /// The file extension for Bash.
    FileExtensionBash,
    /// The file extension for Zsh.
    FileExtensionZsh,
    /// The file extension for env.
    FileExtensionEnv,
    /// The file extension for cj.
    FileExtensionCj,
    /// The file extension for Gitignore.
    FileExtensionGitignore,
    /// The `ma` file extension.
    FileExtensionMa,
    /// The `mac` file extension.
    FileExtensionMac,
    /// The `mads` file extension.
    FileExtensionMads,
    /// The `mag` file extension.
    FileExtensionMag,
    /// The `maker` file extension.
    FileExtensionMaker,
    /// The `man` file extension.
    FileExtensionMan,
    /// The `mar` file extension.
    FileExtensionMar,
    /// The `mathml` file extension.
    FileExtensionMathml,
    /// The `mb` file extension.
    FileExtensionMb,
    /// The `mbk` file extension.
    FileExtensionMbk,
    /// The `mbox` file extension.
    FileExtensionMbox,
    /// The `mc1` file extension.
    FileExtensionMc1,
    /// The `mcd` file extension.
    FileExtensionMcd,
    /// The `mcurl` file extension.
    FileExtensionMcurl,
    /// The `mdb` file extension.
    FileExtensionMdb,
    /// The `mdi` file extension.
    FileExtensionMdi,
    /// The `me` file extension.
    FileExtensionMe,
    /// The `mesh` file extension.
    FileExtensionMesh,
    /// The `meta4` file extension.
    FileExtensionMeta4,
    /// The `metalink` file extension.
    FileExtensionMetalink,
    /// The `mets` file extension.
    FileExtensionMets,
    /// The `mfm` file extension.
    FileExtensionMfm,
    /// The `mft` file extension.
    FileExtensionMft,
    /// The `mgp` file extension.
    FileExtensionMgp,
    /// The `mgz` file extension.
    FileExtensionMgz,
    /// The `mid` file extension.
    FileExtensionMid,
    /// The `midi` file extension.
    FileExtensionMidi,
    /// The `mie` file extension.
    FileExtensionMie,
    /// The `mif` file extension.
    FileExtensionMif,
    /// The `mime` file extension.
    FileExtensionMime,
    /// The `mj2` file extension.
    FileExtensionMj2,
    /// The `mjp2` file extension.
    FileExtensionMjp2,
    /// The `mk3d` file extension.
    FileExtensionMk3d,
    /// The `mka` file extension.
    FileExtensionMka,
    /// The `mks` file extension.
    FileExtensionMks,
    /// The `mkv` file extension.
    FileExtensionMkv,
    /// The `mlp` file extension.
    FileExtensionMlp,
    /// The `mmd` file extension.
    FileExtensionMmd,
    /// The `mmf` file extension.
    FileExtensionMmf,
    /// The `mmr` file extension.
    FileExtensionMmr,
    /// The `mng` file extension.
    FileExtensionMng,
    /// The `mny` file extension.
    FileExtensionMny,
    /// The `mobi` file extension.
    FileExtensionMobi,
    /// The `mods` file extension.
    FileExtensionMods,
    /// The `mov` file extension.
    FileExtensionMov,
    /// The `movie` file extension.
    FileExtensionMovie,
    /// The `mp1` file extension.
    FileExtensionMp1,
    /// The `mp2` file extension.
    FileExtensionMp2,
    /// The `mp21` file extension.
    FileExtensionMp21,
    /// The `mp2a` file extension.
    FileExtensionMp2a,
    /// The `mp3` file extension.
    FileExtensionMp3,
    /// The `mp4` file extension.
    FileExtensionMp4,
    /// The `mp4a` file extension.
    FileExtensionMp4a,
    /// The `mp4s` file extension.
    FileExtensionMp4s,
    /// The `mp4v` file extension.
    FileExtensionMp4v,
    /// The `mpa` file extension.
    FileExtensionMpa,
    /// The `mpc` file extension.
    FileExtensionMpc,
    /// The `mpe` file extension.
    FileExtensionMpe,
    /// The `mpeg` file extension.
    FileExtensionMpeg,
    /// The `mpega` file extension.
    FileExtensionMpega,
    /// The `mpg` file extension.
    FileExtensionMpg,
    /// The `mpg4` file extension.
    FileExtensionMpg4,
    /// The `mpga` file extension.
    FileExtensionMpga,
    /// The `mpkg` file extension.
    FileExtensionMpkg,
    /// The `mpm` file extension.
    FileExtensionMpm,
    /// The `mpn` file extension.
    FileExtensionMpn,
    /// The `mpp` file extension.
    FileExtensionMpp,
    /// The `mpt` file extension.
    FileExtensionMpt,
    /// The `mpv2` file extension.
    FileExtensionMpv2,
    /// The `mpy` file extension.
    FileExtensionMpy,
    /// The `mqy` file extension.
    FileExtensionMqy,
    /// The `mrc` file extension.
    FileExtensionMrc,
    /// The `mrcx` file extension.
    FileExtensionMrcx,
    /// The `ms` file extension.
    FileExtensionMs,
    /// The `mscml` file extension.
    FileExtensionMscml,
    /// The `mseed` file extension.
    FileExtensionMseed,
    /// The `mseq` file extension.
    FileExtensionMseq,
    /// The `msf` file extension.
    FileExtensionMsf,
    /// The `msh` file extension.
    FileExtensionMsh,
    /// The `msi` file extension.
    FileExtensionMsi,
    /// The `msl` file extension.
    FileExtensionMsl,
    /// The `msty` file extension.
    FileExtensionMsty,
    /// The `mts` file extension.
    FileExtensionMts,
    /// The `mus` file extension.
    FileExtensionMus,
    /// The `musicxml` file extension.
    FileExtensionMusicxml,
    /// The `mvb` file extension.
    FileExtensionMvb,
    /// The `mwf` file extension.
    FileExtensionMwf,
    /// The `mxf` file extension.
    FileExtensionMxf,
    /// The `mxl` file extension.
    FileExtensionMxl,
    /// The `mxml` file extension.
    FileExtensionMxml,
    /// The `mxs` file extension.
    FileExtensionMxs,
    /// The `mxu` file extension.
    FileExtensionMxu,
    /// The `n-gage` file extension.
    FileExtensionNGage,
    /// The `n3` file extension.
    FileExtensionN3,
    /// The `nb` file extension.
    FileExtensionNb,
    /// The `nbp` file extension.
    FileExtensionNbp,
    /// The `nc` file extension.
    FileExtensionNc,
    /// The `ncx` file extension.
    FileExtensionNcx,
    /// The `nfo` file extension.
    FileExtensionNfo,
    /// The `ngdat` file extension.
    FileExtensionNgdat,
    /// The `nitf` file extension.
    FileExtensionNitf,
    /// The `nlu` file extension.
    FileExtensionNlu,
    /// The `nml` file extension.
    FileExtensionNml,
    /// The `nnd` file extension.
    FileExtensionNnd,
    /// The `nns` file extension.
    FileExtensionNns,
    /// The `nnw` file extension.
    FileExtensionNnw,
    /// The `npx` file extension.
    FileExtensionNpx,
    /// The `nsc` file extension.
    FileExtensionNsc,
    /// The `nsf` file extension.
    FileExtensionNsf,
    /// The `ntf` file extension.
    FileExtensionNtf,
    /// The `nzb` file extension.
    FileExtensionNzb,
    /// The `oa2` file extension.
    FileExtensionOa2,
    /// The `oa3` file extension.
    FileExtensionOa3,
    /// The `oas` file extension.
    FileExtensionOas,
    /// The `obd` file extension.
    FileExtensionObd,
    /// The `obj` file extension.
    FileExtensionObj,
    /// The `oda` file extension.
    FileExtensionOda,
    /// The `odb` file extension.
    FileExtensionOdb,
    /// The `odc` file extension.
    FileExtensionOdc,
    /// The `odf` file extension.
    FileExtensionOdf,
    /// The `odft` file extension.
    FileExtensionOdft,
    /// The `odg` file extension.
    FileExtensionOdg,
    /// The `odi` file extension.
    FileExtensionOdi,
    /// The `odm` file extension.
    FileExtensionOdm,
    /// The `odp` file extension.
    FileExtensionOdp,
    /// The `ods` file extension.
    FileExtensionOds,
    /// The `odt` file extension.
    FileExtensionOdt,
    /// The `oga` file extension.
    FileExtensionOga,
    /// The `ogg` file extension.
    FileExtensionOgg,
    /// The `ogv` file extension.
    FileExtensionOgv,
    /// The `ogx` file extension.
    FileExtensionOgx,
    /// The `omdoc` file extension.
    FileExtensionOmdoc,
    /// The `onepkg` file extension.
    FileExtensionOnepkg,
    /// The `onetmp` file extension.
    FileExtensionOnetmp,
    /// The `onetoc` file extension.
    FileExtensionOnetoc,
    /// The `onetoc2` file extension.
    FileExtensionOnetoc2,
    /// The `opf` file extension.
    FileExtensionOpf,
    /// The `opml` file extension.
    FileExtensionOpml,
    /// The `oprc` file extension.
    FileExtensionOprc,
    /// The `org` file extension.
    FileExtensionOrg,
    /// The `osf` file extension.
    FileExtensionOsf,
    /// The `osfpvg` file extension.
    FileExtensionOsfpvg,
    /// The `otc` file extension.
    FileExtensionOtc,
    /// The `otf` file extension.
    FileExtensionOtf,
    /// The `otg` file extension.
    FileExtensionOtg,
    /// The `oth` file extension.
    FileExtensionOth,
    /// The `oti` file extension.
    FileExtensionOti,
    /// The `otp` file extension.
    FileExtensionOtp,
    /// The `ots` file extension.
    FileExtensionOts,
    /// The `ott` file extension.
    FileExtensionOtt,
    /// The `oxps` file extension.
    FileExtensionOxps,
    /// The `oxt` file extension.
    FileExtensionOxt,
    /// The `p` file extension.
    FileExtensionP,
    /// The `p10` file extension.
    FileExtensionP10,
    /// The `p12` file extension.
    FileExtensionP12,
    /// The `p7b` file extension.
    FileExtensionP7b,
    /// The `p7c` file extension.
    FileExtensionP7c,
    /// The `p7m` file extension.
    FileExtensionP7m,
    /// The `p7r` file extension.
    FileExtensionP7r,
    /// The `p7s` file extension.
    FileExtensionP7s,
    /// The `p8` file extension.
    FileExtensionP8,
    /// The `pas` file extension.
    FileExtensionPas,
    /// The `paw` file extension.
    FileExtensionPaw,
    /// The `pbd` file extension.
    FileExtensionPbd,
    /// The `pbm` file extension.
    FileExtensionPbm,
    /// The `pcap` file extension.
    FileExtensionPcap,
    /// The `pcf` file extension.
    FileExtensionPcf,
    /// The `pcl` file extension.
    FileExtensionPcl,
    /// The `pclxl` file extension.
    FileExtensionPclxl,
    /// The `pct` file extension.
    FileExtensionPct,
    /// The `pcurl` file extension.
    FileExtensionPcurl,
    /// The `pcx` file extension.
    FileExtensionPcx,
    /// The `pdb` file extension.
    FileExtensionPdb,
    /// The `pdf` file extension.
    FileExtensionPdf,
    /// The `pfa` file extension.
    FileExtensionPfa,
    /// The `pfb` file extension.
    FileExtensionPfb,
    /// The `pfm` file extension.
    FileExtensionPfm,
    /// The `pfr` file extension.
    FileExtensionPfr,
    /// The `pfx` file extension.
    FileExtensionPfx,
    /// The `pgm` file extension.
    FileExtensionPgm,
    /// The `pgn` file extension.
    FileExtensionPgn,
    /// The `pgp` file extension.
    FileExtensionPgp,
    /// The `pic` file extension.
    FileExtensionPic,
    /// The `pict` file extension.
    FileExtensionPict,
    /// The `pkg` file extension.
    FileExtensionPkg,
    /// The `pki` file extension.
    FileExtensionPki,
    /// The `pkipath` file extension.
    FileExtensionPkipath,
    /// The `plb` file extension.
    FileExtensionPlb,
    /// The `plc` file extension.
    FileExtensionPlc,
    /// The `plf` file extension.
    FileExtensionPlf,
    /// The `pls` file extension.
    FileExtensionPls,
    /// The `pml` file extension.
    FileExtensionPml,
    /// The `png` file extension.
    FileExtensionPng,
    /// The `pnm` file extension.
    FileExtensionPnm,
    /// The `pnt` file extension.
    FileExtensionPnt,
    /// The `portpkg` file extension.
    FileExtensionPortpkg,
    /// The `pot` file extension.
    FileExtensionPot,
    /// The `potm` file extension.
    FileExtensionPotm,
    /// The `potx` file extension.
    FileExtensionPotx,
    /// The `ppam` file extension.
    FileExtensionPpam,
    /// The `ppd` file extension.
    FileExtensionPpd,
    /// The `ppm` file extension.
    FileExtensionPpm,
    /// The `pps` file extension.
    FileExtensionPps,
    /// The `ppsm` file extension.
    FileExtensionPpsm,
    /// The `ppsx` file extension.
    FileExtensionPpsx,
    /// The `ppt` file extension.
    FileExtensionPpt,
    /// The `pptm` file extension.
    FileExtensionPptm,
    /// The `pptx` file extension.
    FileExtensionPptx,
    /// The `pqa` file extension.
    FileExtensionPqa,
    /// The `prc` file extension.
    FileExtensionPrc,
    /// The `pre` file extension.
    FileExtensionPre,
    /// The `prf` file extension.
    FileExtensionPrf,
    /// The `ps` file extension.
    FileExtensionPs,
    /// The `psb` file extension.
    FileExtensionPsb,
    /// The `psd` file extension.
    FileExtensionPsd,
    /// The `psf` file extension.
    FileExtensionPsf,
    /// The `pskcxml` file extension.
    FileExtensionPskcxml,
    /// The `ptid` file extension.
    FileExtensionPtid,
    /// The `pub` file extension.
    FileExtensionPub,
    /// The `pvb` file extension.
    FileExtensionPvb,
    /// The `pwn` file extension.
    FileExtensionPwn,
    /// The `pya` file extension.
    FileExtensionPya,
    /// The `pyv` file extension.
    FileExtensionPyv,
    /// The `qam` file extension.
    FileExtensionQam,
    /// The `qbo` file extension.
    FileExtensionQbo,
    /// The `qfx` file extension.
    FileExtensionQfx,
    /// The `qps` file extension.
    FileExtensionQps,
    /// The `qt` file extension.
    FileExtensionQt,
    /// The `qti` file extension.
    FileExtensionQti,
    /// The `qtif` file extension.
    FileExtensionQtif,
    /// The `qwd` file extension.
    FileExtensionQwd,
    /// The `qwt` file extension.
    FileExtensionQwt,
    /// The `qxb` file extension.
    FileExtensionQxb,
    /// The `qxd` file extension.
    FileExtensionQxd,
    /// The `qxl` file extension.
    FileExtensionQxl,
    /// The `qxt` file extension.
    FileExtensionQxt,
    /// The `ra` file extension.
    FileExtensionRa,
    /// The `ram` file extension.
    FileExtensionRam,
    /// The `rar` file extension.
    FileExtensionRar,
    /// The `ras` file extension.
    FileExtensionRas,
    /// The `rcprofile` file extension.
    FileExtensionRcprofile,
    /// The `rdf` file extension.
    FileExtensionRdf,
    /// The `rdz` file extension.
    FileExtensionRdz,
    /// The `rep` file extension.
    FileExtensionRep,
    /// The `res` file extension.
    FileExtensionRes,
    /// The `rgb` file extension.
    FileExtensionRgb,
    /// The `rif` file extension.
    FileExtensionRif,
    /// The `rip` file extension.
    FileExtensionRip,
    /// The `ris` file extension.
    FileExtensionRis,
    /// The `rl` file extension.
    FileExtensionRl,
    /// The `rlc` file extension.
    FileExtensionRlc,
    /// The `rld` file extension.
    FileExtensionRld,
    /// The `rm` file extension.
    FileExtensionRm,
    /// The `rmi` file extension.
    FileExtensionRmi,
    /// The `rmp` file extension.
    FileExtensionRmp,
    /// The `rms` file extension.
    FileExtensionRms,
    /// The `rmvb` file extension.
    FileExtensionRmvb,
    /// The `rnc` file extension.
    FileExtensionRnc,
    /// The `roa` file extension.
    FileExtensionRoa,
    /// The `roff` file extension.
    FileExtensionRoff,
    /// The `rp9` file extension.
    FileExtensionRp9,
    /// The `rpss` file extension.
    FileExtensionRpss,
    /// The `rpst` file extension.
    FileExtensionRpst,
    /// The `rq` file extension.
    FileExtensionRq,
    /// The `rsd` file extension.
    FileExtensionRsd,
    /// The `rss` file extension.
    FileExtensionRss,
    /// The `rtf` file extension.
    FileExtensionRtf,
    /// The `rtx` file extension.
    FileExtensionRtx,
    /// The `s` file extension.
    FileExtensionS,
    /// The `s3m` file extension.
    FileExtensionS3m,
    /// The `saf` file extension.
    FileExtensionSaf,
    /// The `sbml` file extension.
    FileExtensionSbml,
    /// The `sc` file extension.
    FileExtensionSc,
    /// The `scd` file extension.
    FileExtensionScd,
    /// The `scm` file extension.
    FileExtensionScm,
    /// The `scq` file extension.
    FileExtensionScq,
    /// The `scs` file extension.
    FileExtensionScs,
    /// The `scurl` file extension.
    FileExtensionScurl,
    /// The `sda` file extension.
    FileExtensionSda,
    /// The `sdc` file extension.
    FileExtensionSdc,
    /// The `sdd` file extension.
    FileExtensionSdd,
    /// The `sdkd` file extension.
    FileExtensionSdkd,
    /// The `sdkm` file extension.
    FileExtensionSdkm,
    /// The `sdp` file extension.
    FileExtensionSdp,
    /// The `sdw` file extension.
    FileExtensionSdw,
    /// The `see` file extension.
    FileExtensionSee,
    /// The `seed` file extension.
    FileExtensionSeed,
    /// The `sema` file extension.
    FileExtensionSema,
    /// The `semd` file extension.
    FileExtensionSemd,
    /// The `semf` file extension.
    FileExtensionSemf,
    /// The `ser` file extension.
    FileExtensionSer,
    /// The `setpay` file extension.
    FileExtensionSetpay,
    /// The `setreg` file extension.
    FileExtensionSetreg,
    /// The `sfd-hdstx` file extension.
    FileExtensionSfdHdstx,
    /// The `sfs` file extension.
    FileExtensionSfs,
    /// The `sfv` file extension.
    FileExtensionSfv,
    /// The `sgi` file extension.
    FileExtensionSgi,
    /// The `sgl` file extension.
    FileExtensionSgl,
    /// The `sgm` file extension.
    FileExtensionSgm,
    /// The `sgml` file extension.
    FileExtensionSgml,
    /// The `sh` file extension.
    FileExtensionSh,
    /// The `shar` file extension.
    FileExtensionShar,
    /// The `shf` file extension.
    FileExtensionShf,
    /// The `sid` file extension.
    FileExtensionSid,
    /// The `sig` file extension.
    FileExtensionSig,
    /// The `sil` file extension.
    FileExtensionSil,
    /// The `silo` file extension.
    FileExtensionSilo,
    /// The `sis` file extension.
    FileExtensionSis,
    /// The `sisx` file extension.
    FileExtensionSisx,
    /// The `sit` file extension.
    FileExtensionSit,
    /// The `sitx` file extension.
    FileExtensionSitx,
    /// The `skd` file extension.
    FileExtensionSkd,
    /// The `skm` file extension.
    FileExtensionSkm,
    /// The `skp` file extension.
    FileExtensionSkp,
    /// The `skt` file extension.
    FileExtensionSkt,
    /// The `sldm` file extension.
    FileExtensionSldm,
    /// The `sldx` file extension.
    FileExtensionSldx,
    /// The `slt` file extension.
    FileExtensionSlt,
    /// The `sm` file extension.
    FileExtensionSm,
    /// The `smf` file extension.
    FileExtensionSmf,
    /// The `smi` file extension.
    FileExtensionSmi,
    /// The `smil` file extension.
    FileExtensionSmil,
    /// The `smv` file extension.
    FileExtensionSmv,
    /// The `smzip` file extension.
    FileExtensionSmzip,
    /// The `snd` file extension.
    FileExtensionSnd,
    /// The `snf` file extension.
    FileExtensionSnf,
    /// The `so` file extension.
    FileExtensionSo,
    /// The `spc` file extension.
    FileExtensionSpc,
    /// The `spf` file extension.
    FileExtensionSpf,
    /// The `spl` file extension.
    FileExtensionSpl,
    /// The `spot` file extension.
    FileExtensionSpot,
    /// The `spp` file extension.
    FileExtensionSpp,
    /// The `spq` file extension.
    FileExtensionSpq,
    /// The `spx` file extension.
    FileExtensionSpx,
    /// The `sql` file extension.
    FileExtensionSql,
    /// The `src` file extension.
    FileExtensionSrc,
    /// The `srt` file extension.
    FileExtensionSrt,
    /// The `sru` file extension.
    FileExtensionSru,
    /// The `srx` file extension.
    FileExtensionSrx,
    /// The `ssdl` file extension.
    FileExtensionSsdl,
    /// The `sse` file extension.
    FileExtensionSse,
    /// The `ssf` file extension.
    FileExtensionSsf,
    /// The `ssml` file extension.
    FileExtensionSsml,
    /// The `st` file extension.
    FileExtensionSt,
    /// The `stc` file extension.
    FileExtensionStc,
    /// The `std` file extension.
    FileExtensionStd,
    /// The `stf` file extension.
    FileExtensionStf,
    /// The `sti` file extension.
    FileExtensionSti,
    /// The `stk` file extension.
    FileExtensionStk,
    /// The `stl` file extension.
    FileExtensionStl,
    /// The `str` file extension.
    FileExtensionStr,
    /// The `stw` file extension.
    FileExtensionStw,
    /// The `sub` file extension.
    FileExtensionSub,
    /// The `sus` file extension.
    FileExtensionSus,
    /// The `susp` file extension.
    FileExtensionSusp,
    /// The `sv4cpio` file extension.
    FileExtensionSv4cpio,
    /// The `sv4crc` file extension.
    FileExtensionSv4crc,
    /// The `svc` file extension.
    FileExtensionSvc,
    /// The `svd` file extension.
    FileExtensionSvd,
    /// The `svg` file extension.
    FileExtensionSvg,
    /// The `svgz` file extension.
    FileExtensionSvgz,
    /// The `swa` file extension.
    FileExtensionSwa,
    /// The `swf` file extension.
    FileExtensionSwf,
    /// The `swi` file extension.
    FileExtensionSwi,
    /// The `sxc` file extension.
    FileExtensionSxc,
    /// The `sxd` file extension.
    FileExtensionSxd,
    /// The `sxg` file extension.
    FileExtensionSxg,
    /// The `sxi` file extension.
    FileExtensionSxi,
    /// The `sxm` file extension.
    FileExtensionSxm,
    /// The `sxw` file extension.
    FileExtensionSxw,
    /// The `t` file extension.
    FileExtensionT,
    /// The `t3` file extension.
    FileExtensionT3,
    /// The `taglet` file extension.
    FileExtensionTaglet,
    /// The `tao` file extension.
    FileExtensionTao,
    /// The `tar` file extension.
    FileExtensionTar,
    /// The `tcap` file extension.
    FileExtensionTcap,
    /// The `tcl` file extension.
    FileExtensionTcl,
    /// The `teacher` file extension.
    FileExtensionTeacher,
    /// The `tei` file extension.
    FileExtensionTei,
    /// The `teicorpus` file extension.
    FileExtensionTeicorpus,
    /// The `tex` file extension.
    FileExtensionTex,
    /// The `texi` file extension.
    FileExtensionTexi,
    /// The `texinfo` file extension.
    FileExtensionTexinfo,
    /// The `text` file extension.
    FileExtensionText,
    /// The `tfi` file extension.
    FileExtensionTfi,
    /// The `tfm` file extension.
    FileExtensionTfm,
    /// The `tga` file extension.
    FileExtensionTga,
    /// The `thmx` file extension.
    FileExtensionThmx,
    /// The `tif` file extension.
    FileExtensionTif,
    /// The `tiff` file extension.
    FileExtensionTiff,
    /// The `tmo` file extension.
    FileExtensionTmo,
    /// The `torrent` file extension.
    FileExtensionTorrent,
    /// The `tpl` file extension.
    FileExtensionTpl,
    /// The `tpt` file extension.
    FileExtensionTpt,
    /// The `tr` file extension.
    FileExtensionTr,
    /// The `tra` file extension.
    FileExtensionTra,
    /// The `trm` file extension.
    FileExtensionTrm,
    /// The `tsd` file extension.
    FileExtensionTsd,
    /// The `tsv` file extension.
    FileExtensionTsv,
    /// The `ttc` file extension.
    FileExtensionTtc,
    /// The `ttf` file extension.
    FileExtensionTtf,
    /// The `ttl` file extension.
    FileExtensionTtl,
    /// The `twd` file extension.
    FileExtensionTwd,
    /// The `twds` file extension.
    FileExtensionTwds,
    /// The `txd` file extension.
    FileExtensionTxd,
    /// The `txf` file extension.
    FileExtensionTxf,
    /// The `txt` file extension.
    FileExtensionTxt,
    /// The `u32` file extension.
    FileExtensionU32,
    /// The `udeb` file extension.
    FileExtensionUdeb,
    /// The `ufd` file extension.
    FileExtensionUfd,
    /// The `ufdl` file extension.
    FileExtensionUfdl,
    /// The `ulw` file extension.
    FileExtensionUlw,
    /// The `ulx` file extension.
    FileExtensionUlx,
    /// The `umj` file extension.
    FileExtensionUmj,
    /// The `unityweb` file extension.
    FileExtensionUnityweb,
    /// The `uoml` file extension.
    FileExtensionUoml,
    /// The `uri` file extension.
    FileExtensionUri,
    /// The `uris` file extension.
    FileExtensionUris,
    /// The `urls` file extension.
    FileExtensionUrls,
    /// The `ustar` file extension.
    FileExtensionUstar,
    /// The `utz` file extension.
    FileExtensionUtz,
    /// The `uu` file extension.
    FileExtensionUu,
    /// The `uva` file extension.
    FileExtensionUva,
    /// The `uvd` file extension.
    FileExtensionUvd,
    /// The `uvf` file extension.
    FileExtensionUvf,
    /// The `uvg` file extension.
    FileExtensionUvg,
    /// The `uvh` file extension.
    FileExtensionUvh,
    /// The `uvi` file extension.
    FileExtensionUvi,
    /// The `uvm` file extension.
    FileExtensionUvm,
    /// The `uvp` file extension.
    FileExtensionUvp,
    /// The `uvs` file extension.
    FileExtensionUvs,
    /// The `uvt` file extension.
    FileExtensionUvt,
    /// The `uvu` file extension.
    FileExtensionUvu,
    /// The `uvv` file extension.
    FileExtensionUvv,
    /// The `uvva` file extension.
    FileExtensionUvva,
    /// The `uvvd` file extension.
    FileExtensionUvvd,
    /// The `uvvf` file extension.
    FileExtensionUvvf,
    /// The `uvvg` file extension.
    FileExtensionUvvg,
    /// The `uvvh` file extension.
    FileExtensionUvvh,
    /// The `uvvi` file extension.
    FileExtensionUvvi,
    /// The `uvvm` file extension.
    FileExtensionUvvm,
    /// The `uvvp` file extension.
    FileExtensionUvvp,
    /// The `uvvs` file extension.
    FileExtensionUvvs,
    /// The `uvvt` file extension.
    FileExtensionUvvt,
    /// The `uvvu` file extension.
    FileExtensionUvvu,
    /// The `uvvv` file extension.
    FileExtensionUvvv,
    /// The `uvvx` file extension.
    FileExtensionUvvx,
    /// The `uvvz` file extension.
    FileExtensionUvvz,
    /// The `uvx` file extension.
    FileExtensionUvx,
    /// The `uvz` file extension.
    FileExtensionUvz,
    /// The `vcard` file extension.
    FileExtensionVcard,
    /// The `vcd` file extension.
    FileExtensionVcd,
    /// The `vcf` file extension.
    FileExtensionVcf,
    /// The `vcg` file extension.
    FileExtensionVcg,
    /// The `vcs` file extension.
    FileExtensionVcs,
    /// The `vcx` file extension.
    FileExtensionVcx,
    /// The `vis` file extension.
    FileExtensionVis,
    /// The `viv` file extension.
    FileExtensionViv,
    /// The `vob` file extension.
    FileExtensionVob,
    /// The `vor` file extension.
    FileExtensionVor,
    /// The `vox` file extension.
    FileExtensionVox,
    /// The `vrml` file extension.
    FileExtensionVrml,
    /// The `vsd` file extension.
    FileExtensionVsd,
    /// The `vsf` file extension.
    FileExtensionVsf,
    /// The `vss` file extension.
    FileExtensionVss,
    /// The `vst` file extension.
    FileExtensionVst,
    /// The `vsw` file extension.
    FileExtensionVsw,
    /// The `vtu` file extension.
    FileExtensionVtu,
    /// The `vxml` file extension.
    FileExtensionVxml,
    /// The `w3d` file extension.
    FileExtensionW3d,
    /// The `wad` file extension.
    FileExtensionWad,
    /// The `wav` file extension.
    FileExtensionWav,
    /// The `wax` file extension.
    FileExtensionWax,
    /// The `wbmp` file extension.
    FileExtensionWbmp,
    /// The `wbs` file extension.
    FileExtensionWbs,
    /// The `wbxml` file extension.
    FileExtensionWbxml,
    /// The `wcm` file extension.
    FileExtensionWcm,
    /// The `wdb` file extension.
    FileExtensionWdb,
    /// The `wdp` file extension.
    FileExtensionWdp,
    /// The `weba` file extension.
    FileExtensionWeba,
    /// The `webm` file extension.
    FileExtensionWebm,
    /// The `webp` file extension.
    FileExtensionWebp,
    /// The `wasm` file extension.
    FileExtensionWasm,
    /// The `wg` file extension.
    FileExtensionWg,
    /// The `wgt` file extension.
    FileExtensionWgt,
    /// The `wks` file extension.
    FileExtensionWks,
    /// The `wm` file extension.
    FileExtensionWm,
    /// The `wma` file extension.
    FileExtensionWma,
    /// The `wmd` file extension.
    FileExtensionWmd,
    /// The `wmf` file extension.
    FileExtensionWmf,
    /// The `wml` file extension.
    FileExtensionWml,
    /// The `wmlc` file extension.
    FileExtensionWmlc,
    /// The `wmls` file extension.
    FileExtensionWmls,
    /// The `wmlsc` file extension.
    FileExtensionWmlsc,
    /// The `wmv` file extension.
    FileExtensionWmv,
    /// The `wmx` file extension.
    FileExtensionWmx,
    /// The `wmz` file extension.
    FileExtensionWmz,
    /// The `woff` file extension.
    FileExtensionWoff,
    /// The `woff2` file extension.
    FileExtensionWoff2,
    /// The `wpd` file extension.
    FileExtensionWpd,
    /// The `wpl` file extension.
    FileExtensionWpl,
    /// The `wps` file extension.
    FileExtensionWps,
    /// The `wqd` file extension.
    FileExtensionWqd,
    /// The `wri` file extension.
    FileExtensionWri,
    /// The `wrl` file extension.
    FileExtensionWrl,
    /// The `wsdl` file extension.
    FileExtensionWsdl,
    /// The `wspolicy` file extension.
    FileExtensionWspolicy,
    /// The `wtb` file extension.
    FileExtensionWtb,
    /// The `wvx` file extension.
    FileExtensionWvx,
    /// The `x32` file extension.
    FileExtensionX32,
    /// The `x3d` file extension.
    FileExtensionX3d,
    /// The `x3db` file extension.
    FileExtensionX3db,
    /// The `x3dbz` file extension.
    FileExtensionX3dbz,
    /// The `x3dv` file extension.
    FileExtensionX3dv,
    /// The `x3dvz` file extension.
    FileExtensionX3dvz,
    /// The `x3dz` file extension.
    FileExtensionX3dz,
    /// The `xaml` file extension.
    FileExtensionXaml,
    /// The `xap` file extension.
    FileExtensionXap,
    /// The `xar` file extension.
    FileExtensionXar,
    /// The `xbap` file extension.
    FileExtensionXbap,
    /// The `xbd` file extension.
    FileExtensionXbd,
    /// The `xbm` file extension.
    FileExtensionXbm,
    /// The `xdf` file extension.
    FileExtensionXdf,
    /// The `xdm` file extension.
    FileExtensionXdm,
    /// The `xdp` file extension.
    FileExtensionXdp,
    /// The `xdssc` file extension.
    FileExtensionXdssc,
    /// The `xdw` file extension.
    FileExtensionXdw,
    /// The `xenc` file extension.
    FileExtensionXenc,
    /// The `xer` file extension.
    FileExtensionXer,
    /// The `xfdf` file extension.
    FileExtensionXfdf,
    /// The `xfdl` file extension.
    FileExtensionXfdl,
    /// The `xht` file extension.
    FileExtensionXht,
    /// The `xhtml` file extension.
    FileExtensionXhtml,
    /// The `xhvml` file extension.
    FileExtensionXhvml,
    /// The `xif` file extension.
    FileExtensionXif,
    /// The `xla` file extension.
    FileExtensionXla,
    /// The `xlam` file extension.
    FileExtensionXlam,
    /// The `xlc` file extension.
    FileExtensionXlc,
    /// The `xlf` file extension.
    FileExtensionXlf,
    /// The `xlm` file extension.
    FileExtensionXlm,
    /// The `xls` file extension.
    FileExtensionXls,
    /// The `xlsb` file extension.
    FileExtensionXlsb,
    /// The `xlsm` file extension.
    FileExtensionXlsm,
    /// The `xlsx` file extension.
    FileExtensionXlsx,
    /// The `xlt` file extension.
    FileExtensionXlt,
    /// The `xltm` file extension.
    FileExtensionXltm,
    /// The `xltx` file extension.
    FileExtensionXltx,
    /// The `xlw` file extension.
    FileExtensionXlw,
    /// The `xm` file extension.
    FileExtensionXm,
    /// The `xml` file extension.
    FileExtensionXml,
    /// The `xo` file extension.
    FileExtensionXo,
    /// The `xop` file extension.
    FileExtensionXop,
    /// The `xpi` file extension.
    FileExtensionXpi,
    /// The `xpl` file extension.
    FileExtensionXpl,
    /// The `xpm` file extension.
    FileExtensionXpm,
    /// The `xpr` file extension.
    FileExtensionXpr,
    /// The `xps` file extension.
    FileExtensionXps,
    /// The `xpw` file extension.
    FileExtensionXpw,
    /// The `xpx` file extension.
    FileExtensionXpx,
    /// The `xsl` file extension.
    FileExtensionXsl,
    /// The `xslt` file extension.
    FileExtensionXslt,
    /// The `xsm` file extension.
    FileExtensionXsm,
    /// The `xspf` file extension.
    FileExtensionXspf,
    /// The `xul` file extension.
    FileExtensionXul,
    /// The `xvm` file extension.
    FileExtensionXvm,
    /// The `xvml` file extension.
    FileExtensionXvml,
    /// The `xwd` file extension.
    FileExtensionXwd,
    /// The `xyz` file extension.
    FileExtensionXyz,
    /// The `xz` file extension.
    FileExtensionXz,
    /// The `yang` file extension.
    FileExtensionYang,
    /// The `yin` file extension.
    FileExtensionYin,
    /// The `z` file extension.
    FileExtensionZ,
    /// The `Z` file extension.
    FileExtensionZUppercasee,
    /// The `z1` file extension.
    FileExtensionZ1,
    /// The `z2` file extension.
    FileExtensionZ2,
    /// The `z3` file extension.
    FileExtensionZ3,
    /// The `z4` file extension.
    FileExtensionZ4,
    /// The `z5` file extension.
    FileExtensionZ5,
    /// The `z6` file extension.
    FileExtensionZ6,
    /// The `z7` file extension.
    FileExtensionZ7,
    /// The `z8` file extension.
    FileExtensionZ8,
    /// The `zaz` file extension.
    FileExtensionZaz,
    /// The `zip` file extension.
    FileExtensionZip,
    /// The `zir` file extension.
    FileExtensionZir,
    /// The `zirz` file extension.
    FileExtensionZirz,
    /// The `zmm` file extension.
    FileExtensionZmm,
    /// An empty or unknown file extension.
    #[default]
    Unknown,
}
