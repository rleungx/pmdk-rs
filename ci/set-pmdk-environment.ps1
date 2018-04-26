$library_path = [Environment]::GetEnvironmentVariable("PMDK_LibraryPath","Machine")
$include_path = [Environment]::GetEnvironmentVariable("PMDK_IncludePath","Machine")

if ($env:target.Contains("msvc")) {
     [Environment]::SetEnvironmentVariable("LIB", "${env:LIB};$library_path", "Machine")
     [Environment]::SetEnvironmentVariable("INCLUDE", "${env:INCLUDE};$include_path", "Machine")
}
if ($env:target.Contains("gnu")) {
     [Environment]::SetEnvironmentVariable("LIBRARY_PATH", "${env:LIBRARY_PATH};$library_path", "Machine")
     [Environment]::SetEnvironmentVariable("C_INCLUDE_PATH", "${env:C_INCLUDE_PATH};$include_path", "Machine")
}