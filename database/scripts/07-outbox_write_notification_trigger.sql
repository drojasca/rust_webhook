-- Create a notification function
CREATE OR REPLACE FUNCTION notify_data_change()
RETURNS TRIGGER AS $$
DECLARE
    payload JSON;
BEGIN
    -- Build payload with change information
    payload := json_build_object(
        'table', TG_TABLE_NAME,
        'data', row_to_json(NEW)
    );

    -- Send notification
    PERFORM pg_notify('data_changes', payload::text);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_notify_data_change ON outbox;

-- Apply trigger to table
CREATE TRIGGER trg_notify_data_change
AFTER INSERT ON outbox
FOR EACH ROW
EXECUTE FUNCTION notify_data_change();