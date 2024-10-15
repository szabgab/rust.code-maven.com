-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: dance
-- ------------------------------

DEFINE TABLE dance SCHEMALESS PERMISSIONS NONE;

-- ------------------------------
-- TABLE: student
-- ------------------------------

DEFINE TABLE student SCHEMALESS PERMISSIONS NONE;

-- ------------------------------
-- TRANSACTION
-- ------------------------------

BEGIN TRANSACTION;

-- ------------------------------
-- TABLE DATA: dance
-- ------------------------------

UPDATE dance:c4v2dpfzsreg1u5nleeq CONTENT { id: dance:c4v2dpfzsreg1u5nleeq, name: 'Flamenco' };
UPDATE dance:gjjhx7vr62vaiqr49ivr CONTENT { id: dance:gjjhx7vr62vaiqr49ivr, name: 'Introduction to Dancing' };

-- ------------------------------
-- TABLE DATA: student
-- ------------------------------

UPDATE student:h8c5rpmyhb3p3l0yu7al CONTENT { classes: [dance:c4v2dpfzsreg1u5nleeq, dance:gjjhx7vr62vaiqr49ivr], id: student:h8c5rpmyhb3p3l0yu7al, name: 'Jane Doe' };

-- ------------------------------
-- TRANSACTION
-- ------------------------------

COMMIT TRANSACTION;

