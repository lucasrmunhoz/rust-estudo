// Constantes arquiteturais canonicas do VNP.
// Fonte unica de verdade: nenhum binario redeclara estes valores.

// Aresta do chunk, em voxels. Decisao de projeto -- tudo abaixo deriva daqui.
pub const CHUNK_SIDE: usize = 32;

// Area de uma camada do chunk. Derivada em compile-time, nunca digitada a mao.
pub const CHUNK_AREA: usize = CHUNK_SIDE * CHUNK_SIDE;

// Total de voxels de um chunk. Derivada -- dimensiona o array de armazenamento.
pub const CHUNK_VOXEL_COUNT: usize = CHUNK_SIDE * CHUNK_AREA;

// Aresta do chunk no mundo das coordenadas (i32), para div_euclid/rem_euclid.
// as: estreitamento, avaliado em compile-time -- 32 cabe com folga em i32.
pub const CHUNK_SIDE_I32: i32 = CHUNK_SIDE as i32;

// ID reservado para o vazio. Solidez e derivada disto, nunca armazenada.
pub const VOXEL_ID_AIR: u8 = 0;
