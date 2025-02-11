INSERT INTO
  roles (name)
VALUES
  ('Admin'),
  ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
  users (name, email, password_hash, role_id)
SELECT
  'Eleazar Fig',
  'eleazar.fig@example.com',
  '$2b$12$rnpTMDnlSJ9bVFnhv5oMn.C/qEtm6xzeaPRzj0JsPb8Be9j9X9tWe',
  role_id
FROM
  roles
WHERE
  name LIKE 'Admin';
