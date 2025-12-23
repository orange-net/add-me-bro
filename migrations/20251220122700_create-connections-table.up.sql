-- Add down migration script here
--
CREATE TABLE connections (
  id SERIAL PRIMARY KEY,
  initiator_uid int NOT NULL REFERENCES users(user_id),
  recipient_uid  int NOT NULL REFERENCES users(user_id),
  status varchar(50) NOT NULL DEFAULT 'Pending',
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);


CREATE TRIGGER update_modified_time BEFORE UPDATE ON connections FOR EACH ROW EXECUTE PROCEDURE update_modified_column();
