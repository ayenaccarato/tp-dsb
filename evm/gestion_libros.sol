// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

contract DigitalLibrary {
    struct Libro {
        uint256 id;
        string titulo;
        string descripcion;
        address propietario;
    }

    mapping(uint256 => Libro) private libros;
    mapping(address => uint256[]) private librosPorUsuario;
    mapping(uint256 => bool) private existeLibro;

    function registrarLibro(
        uint256 _id,
        string memory _titulo,
        string memory _descripcion
    ) public {
        require(!existeLibro[_id], "El libro ya existe");

        libros[_id] = Libro({
            id: _id,
            titulo: _titulo,
            descripcion: _descripcion,
            propietario: msg.sender
        });

        librosPorUsuario[msg.sender].push(_id);
        existeLibro[_id] = true;
    }

    function actualizarLibro(
        uint256 _id,
        string memory _nuevoTitulo,
        string memory _nuevaDescripcion
    ) public {
        require(existeLibro[_id], "Libro inexistente");
        require(libros[_id].propietario == msg.sender, "No sos el propietario");

        libros[_id].titulo = _nuevoTitulo;
        libros[_id].descripcion = _nuevaDescripcion;
    }

    function transferirLibro(uint256 _id, address _nuevoPropietario) public {
        require(existeLibro[_id], "Libro inexistente");
        require(libros[_id].propietario == msg.sender, "No sos el propietario");

        libros[_id].propietario = _nuevoPropietario;
        librosPorUsuario[_nuevoPropietario].push(_id);
    }

    function obtenerLibro(uint256 _id) public view returns (Libro memory) {
        require(existeLibro[_id], "Libro inexistente");
        return libros[_id];
    }

    function librosDeUsuario(address _usuario)
        public
        view
        returns (uint256[] memory)
    {
        return librosPorUsuario[_usuario];
    }
}
