@echo off
for %%f in (trsp\*.dat) do echo %%~nxf & target\debug\otim < %%f