@echo off
rem Gera dist\Instalar Tenkaichi 3.exe em segundo plano (janela minimizada, prioridade baixa), testando a build
rem antes (testar_build.py). O andamento fica em dist\gerar.log; a ultima linha diz FIM e o resultado.
if not "%1"=="min" (start "" /min /low cmd /c "%~f0" min ^& exit /b)
cd /d "%~dp0..\.."
set PY=python
if exist ".venv\Scripts\python.exe" set PY=.venv\Scripts\python.exe
if not exist dist mkdir dist
echo inicio %date% %time% > dist\gerar.log
%PY% tools\install\testar_build.py >> dist\gerar.log 2>&1
if errorlevel 1 (echo FIM teste falhou, instalador nao gerado %date% %time% >> dist\gerar.log & exit /b 1)
%PY% tools\install\empacotar.py >> dist\gerar.log 2>&1
echo FIM codigo %errorlevel% %date% %time% >> dist\gerar.log
