const helloWorld = {
  hello: async ({ to }: { to: string }) => {
    const result = ["Hello", to];
    return { result };
  }
};

export default helloWorld; 