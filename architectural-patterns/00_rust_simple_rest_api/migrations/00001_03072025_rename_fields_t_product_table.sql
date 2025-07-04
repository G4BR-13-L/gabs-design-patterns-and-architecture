-- Rename table
ALTER TABLE t_produto RENAME TO t_product;

-- Rename columns
ALTER TABLE t_product RENAME COLUMN nome TO name;
ALTER TABLE t_product RENAME COLUMN descricao TO description;
ALTER TABLE t_product RENAME COLUMN valor TO price;
ALTER TABLE t_product RENAME COLUMN data_cadastro TO created_at;
ALTER TABLE t_product RENAME COLUMN ativo TO active;
ALTER TABLE t_product RENAME COLUMN disponivel TO available;
