CREATE OR REPLACE FUNCTION write_to_outbox()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO outbox (event_id)
    VALUES (NEW.event_id);
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trig_write_to_outbox ON events;

CREATE TRIGGER trig_write_to_outbox
AFTER INSERT ON events
FOR EACH ROW
EXECUTE FUNCTION write_to_outbox();