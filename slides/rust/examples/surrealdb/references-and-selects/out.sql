-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: course
-- ------------------------------

DEFINE TABLE course TYPE ANY SCHEMALESS PERMISSIONS NONE;

-- ------------------------------
-- TABLE: student
-- ------------------------------

DEFINE TABLE student TYPE ANY SCHEMALESS PERMISSIONS NONE;

-- ------------------------------
-- TABLE DATA: course
-- ------------------------------

INSERT [ { id: course:hlj49mis0u0vbf9amnqr, name: 'Chemistry' }, { id: course:meq8szcqvtuq4or9nu6h, name: 'Physics' }, { id: course:naof29apn50iepz0kqy4, name: 'Biology' } ];

-- ------------------------------
-- TABLE DATA: student
-- ------------------------------

INSERT [ { courses: [course:naof29apn50iepz0kqy4, course:hlj49mis0u0vbf9amnqr, course:meq8szcqvtuq4or9nu6h], id: student:i75save2cwar0v994a8z, name: 'Jane Doe' } ];

