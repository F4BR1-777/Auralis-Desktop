# Auralis Desktop — edición preliminar
**Feel every note.** Cliente musical nativo para Windows, derivado de BitChord PC Official Rust 2.0.1. No es una aplicación de Google ni de Apple. Esta revisión es código fuente sin compilar, no un instalador listo.

## Cambios de esta revisión
- Identidad Auralis: icono, marca, ventana, bandeja y accesos directos.
- Ajustes de inspiración Liquid Glass: reproductor flotante por defecto, mayor transparencia y reflejos, radios y colores. No implementa la refracción óptica del material nativo de Apple.
- Navegación y búsqueda parcialmente traducidas al español.
- Nombre de ejecutable, ID del instalador, protocolo local y almacén de datos/credenciales separados del original.
- Actualizaciones automáticas heredadas desactivadas para no instalar binarios de otros proyectos. Discord desactivado por defecto.
- Sin alteraciones al mecanismo de reproducción, búsqueda, autenticación o sincronización del motor original. No se han agregado mecanismos para eludir restricciones de servicios.

## Preparar y compilar (Windows x64)
Conserva el ZIP original `BitChord-PC-Official-rust-2.0.zip`. El kit contiene únicamente los archivos modificados, la licencia y scripts; el original se usa localmente para reconstruir toda la fuente, incluidos los recursos que necesita el compilador.

1. Extrae el kit en una carpeta propia y coloca allí el ZIP original (o déjalo en Descargas).
2. Instala las herramientas de desarrollo necesarias: Rust estable con target MSVC x64, Visual Studio Build Tools con C++ para escritorio y Windows SDK. Para generar un instalador, también Inno Setup 6. Windows debe contar con Microsoft Edge WebView2 Runtime para el inicio de sesión heredado. Los instaladores deben proceder de sus editores oficiales.
3. Abre PowerShell en la carpeta del kit; ejecuta `.\COMPILAR.ps1`. También puedes ejecutar solamente la preparación y después compilar.

```powershell
.\COMPILAR.ps1
# O, para preparar sin compilar:
.\COMPILAR.ps1 -SoloPreparar
# Si la fuente ya existe y necesitas reintentar la compilación:
.\CREAR-EXE.ps1
```

El script verifica SHA-256 del ZIP de origen antes de copiar cambios. No modifica el ZIP, no instala dependencias del sistema ni eleva permisos. Cargo sí necesita acceso a sus dependencias de compilación. Si PowerShell o las políticas de tu equipo restringen scripts, no desactives la protección; puedes revisar los scripts y ejecutar sus pasos desde una consola autorizada.

Resultados esperados, **solo después de una compilación exitosa**:
- `Auralis-Portable\auralis.exe` y `WebView2Loader.dll`.
- `fuente\BitChord-PC-Official-rust-2.0\installer\Output\Auralis-Setup-2.0.1.exe` si Inno Setup está disponible.

No se incluye binario compilado ni se ha probado Windows. No tiene firma digital. Prueba primero el programa sin iniciar sesión. La aplicación hereda una integración no oficial de YouTube Music: la ventana de Google del proyecto original maneja cookies de sesión; en Windows el almacenamiento persistente utiliza Credential Manager, pero eso no equivale a OAuth ni demuestra que el cliente sea seguro. No recomendamos iniciar sesión en una cuenta principal hasta auditar el código y las dependencias, verificar la procedencia y probar en un entorno apropiado. Las funciones y disponibilidad dependen de YouTube Music. No hay garantía de acceso a todo su catálogo, ni de disponibilidad de funciones de pago.

## Licencia y procedencia
Derivado del ZIP proporcionado por el usuario: BitChord-PC-Official-rust-2.0, paquete 2.0.1 (GPL-3.0-only). El README original y la licencia original se conservan. Repositorio señalado por el proyecto: https://github.com/jackychan4891/BitChord-PC-Continued . Los autores y las licencias de terceros mantienen sus créditos. Cambios de marca y presentación de Auralis: noviembre de 2026. Al distribuir un binario hay que cumplir GPLv3, proporcionar el código fuente correspondiente y verificar las licencias de recursos de terceros. No presentarlo como desarrollo original íntegro.

## Pruebas pendientes
Compilación real con Rust en Windows; arranque; búsqueda y reproducción sin iniciar sesión; comportamiento de inicio/cierre de sesión tras auditoría; sincronización de favoritos y playlists; accesibilidad, rendimiento, instalador y revisión de dependencias. Las pruebas estáticas del kit no sustituyen estas pruebas.
