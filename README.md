# Gestor-de-personajes-en-videojuegos
Proyecto de Solana

Descripcion

Este proyecto es un smart contract desarrollado en Rust usando el framework Anchor para la blockchain de Solana. El programa permite a un usuario crear y administrar una colección de personajes de videojuegos, donde puede agregar, visualizar, eliminar y modificar el estado de sus personajes y cada usuario tiene su propia colección utilizando PDA (Program Derived Address), lo que permite que los datos sean únicos y seguros para cada propietario.

Tecnologías utilizadas
~Rust
~Anchor Framework
~Solana Blockchain


Esta basado en:

C-REATE
R-EAD
U-PDATE
D-ELETE


Uso de PDA


El proyecto utiliza Program Derived Address (PDA) para crear una cuenta única para cada usuario y la dirección se genera utilizando:

seeds = [b"coleccion", owner.key().as_ref()]

Esto asegura que cada usuario tenga su propia colección independiente.



Cómo usar el proyecto


1️. Clonar el repositorio

git clone URL_DEL_REPOSITORIO

2️. Compilar el programa

anchor build

3️. Ejecutar pruebas

anchor test




Autor

Proyecto desarrollado por mi, CARLOS EDUARDO SANCHEZ GARCIA como proyecto/practica de desarrollo en la blockchain de Solana usando el lenguiaje Rust para la certificacion de la misma.