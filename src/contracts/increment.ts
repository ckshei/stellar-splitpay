const incrementor = {
  options: {
    publicKey: "",
    signTransaction: null,
  },
  
  increment: async function() {
    // Implementación simulada del incrementor
    return {
      signAndSend: async function() {
        return { result: 1 };
      }
    };
  }
};

export default incrementor; 