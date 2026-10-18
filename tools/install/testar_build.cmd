@echo off
rem Testa a build do jogo antes de gerar o instalador (tools\install\testar_build.py): compila, joga a tela
rem inicial e uma luta numa janela escondida e confere as imagens. Resultado no fim; imagens em dist\teste.
cd /d "%~dp0..\.."
set PY=python
if exist ".venv\Scripts\python.exe" set PY=.venv\Scripts\python.exe
%PY% tools\install\testar_build.py %*
set R=%errorlevel%
echo.
if "%R%"=="0" (echo Build OK.) else (echo A build tem problema: veja acima.)
if not "%BT3_SEM_PAUSA%"=="1" pause
exit /b %R%
