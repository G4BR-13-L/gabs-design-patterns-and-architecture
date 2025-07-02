public class User {

    private final String name;
    private final String email;
    private final Integer age;

    private User(Builder builder) {
        this.name = builder.name;
        this.email = builder.email;
        this.age = builder.age;
    }

    public static class Builder {

        private final String name;
        private String email;
        private Integer age;

        public Builder(String name) {
            this.name = name;
        }

        public Builder withEmail(String email) {
            this.email = email;
            return this;
        }

        public Builder withAge(int age) {
            this.age = age;
            return this;
        }

        public User build() {
            return new User(this);
        }
    }

    @Override
    public String toString() {
        return (
            "User{name='" + name + "', email='" + email + "', age=" + age + "}"
        );
    }

    public static void main(String[] args) {
        User user1 = new User.Builder("Gabriel")
            .withEmail("gabrieluser@email.com")
            .withAge(30)
            .build();

        User user2 = new User.Builder("Lucas").build();

        System.out.println(user1);
        System.out.println(user2);
    }
}
