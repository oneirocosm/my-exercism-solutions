{-# LANGUAGE CPP #-}
{-# LANGUAGE NoRebindableSyntax #-}
{-# OPTIONS_GHC -fno-warn-missing-import-lists #-}
module Paths_hello_world (
    version,
    getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir,
    getDataFileName, getSysconfDir
  ) where

import qualified Control.Exception as Exception
import Data.Version (Version(..))
import System.Environment (getEnv)
import Prelude

#if defined(VERSION_base)

#if MIN_VERSION_base(4,0,0)
catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
#else
catchIO :: IO a -> (Exception.Exception -> IO a) -> IO a
#endif

#else
catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
#endif
catchIO = Exception.catch

version :: Version
version = Version [1,1,0,5] []
bindir, libdir, dynlibdir, datadir, libexecdir, sysconfdir :: FilePath

bindir     = "/home/colton/Projects/exercism/haskell/hello-world/.stack-work/install/x86_64-linux-tinfo6/d618d9ef45c856ce62c0d645f41155fa7407b1fbe7280a3150d0ea574886216c/8.8.3/bin"
libdir     = "/home/colton/Projects/exercism/haskell/hello-world/.stack-work/install/x86_64-linux-tinfo6/d618d9ef45c856ce62c0d645f41155fa7407b1fbe7280a3150d0ea574886216c/8.8.3/lib/x86_64-linux-ghc-8.8.3/hello-world-1.1.0.5-FFFbfDavzwU2PHLyFlpGf-test"
dynlibdir  = "/home/colton/Projects/exercism/haskell/hello-world/.stack-work/install/x86_64-linux-tinfo6/d618d9ef45c856ce62c0d645f41155fa7407b1fbe7280a3150d0ea574886216c/8.8.3/lib/x86_64-linux-ghc-8.8.3"
datadir    = "/home/colton/Projects/exercism/haskell/hello-world/.stack-work/install/x86_64-linux-tinfo6/d618d9ef45c856ce62c0d645f41155fa7407b1fbe7280a3150d0ea574886216c/8.8.3/share/x86_64-linux-ghc-8.8.3/hello-world-1.1.0.5"
libexecdir = "/home/colton/Projects/exercism/haskell/hello-world/.stack-work/install/x86_64-linux-tinfo6/d618d9ef45c856ce62c0d645f41155fa7407b1fbe7280a3150d0ea574886216c/8.8.3/libexec/x86_64-linux-ghc-8.8.3/hello-world-1.1.0.5"
sysconfdir = "/home/colton/Projects/exercism/haskell/hello-world/.stack-work/install/x86_64-linux-tinfo6/d618d9ef45c856ce62c0d645f41155fa7407b1fbe7280a3150d0ea574886216c/8.8.3/etc"

getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir, getSysconfDir :: IO FilePath
getBinDir = catchIO (getEnv "hello_world_bindir") (\_ -> return bindir)
getLibDir = catchIO (getEnv "hello_world_libdir") (\_ -> return libdir)
getDynLibDir = catchIO (getEnv "hello_world_dynlibdir") (\_ -> return dynlibdir)
getDataDir = catchIO (getEnv "hello_world_datadir") (\_ -> return datadir)
getLibexecDir = catchIO (getEnv "hello_world_libexecdir") (\_ -> return libexecdir)
getSysconfDir = catchIO (getEnv "hello_world_sysconfdir") (\_ -> return sysconfdir)

getDataFileName :: FilePath -> IO FilePath
getDataFileName name = do
  dir <- getDataDir
  return (dir ++ "/" ++ name)
