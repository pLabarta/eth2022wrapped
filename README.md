# Your 2022 Ethereum Wrapped

**Titulo**: Your 2022 Ethereum Wrapped: Un sitio que permite obtener estadisticas generadas a lo largo del año para un address.
Description: A brief overview of the project, including its main features and how it is used.

**Descripción**: El proyecto "Your 2022 Ethereum Wrapped" es un sitio web que permite a los usuarios obtener estadísticas generadas a lo largo del año para una dirección de Ethereum. El sitio utiliza la API de flipsidecrypto para recopilar los datos relacionados con una dirección de Ethereum y presentarlos de manera gráfica. El sitio está construido con el framework de frontend Yew para Rust, y utiliza gráficos construidos con d3.js y herramientas de Rust para trabajar con WebAssembly e importar librerías del ecosistema de JavaScript. Este proyecto es ideal para aquellos interesados en monitorear y analizar los movimientos de sus direcciones de Ethereum a lo largo del año 2022.

**Instalación**

**1.** Instala rust si no está instalado, puedes instalarlo con el comando

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. Instalar mediante cargo las herramientas Trunk y wasm-bindgen-cli, que se pueden instalar mediante el comando

`cargo install trunk wasm-bindgen-cli`

Cargo es el gestor de paquetes de Rust y se usa para instalar las herramientas necesarias para el proyecto.

3. Añadir el target de compilación wasm32-unknown-unknown. Este target es necesario para poder compilar código de Rust a WebAssembly. Puedes añadirlo con el comando

`rustup target add wasm32-unknown-unknown`

4. Ejecutar el comando

`trunk serve`

Uso: La address está hardcodeada en el codigo, para ingresar el address que se busque explorar, se debe editar el codigo en el archivo main.rs y compilar con trunk server.

Documentacion:
Flipside Schroom SDK: https://docs.flipsidecrypto.com/shroomdk-sdk/get-started

License: A statement indicating the license under which the project is released.
