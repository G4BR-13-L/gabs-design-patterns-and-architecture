public class Logger {

    private static Logger instance;
    private String level = "INFO";

    private Logger() {}

    public static synchronized Logger getInstance() {
        if (instance == null) {
            instance = new Logger();
        }
        return instance;
    }

    public void log(String message) {
        System.out.println("[" + level + "] " + message);
    }

    public void setLevel(String level) {
        this.level = level;
    }

    public static void main(String[] args) {
        Logger logger1 = Logger.getInstance();
        logger1.log("Starting application...");
        logger1.setLevel("DEBUG");

        Logger logger2 = Logger.getInstance();
        logger2.log("This is a debug message.");
    }
}
