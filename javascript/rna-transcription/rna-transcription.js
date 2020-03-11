export const toRna = (dna) => {
  return dna.split('')
    .map(nucleo => {
      const rna = {
        G: 'C',
        C: 'G',
        T: 'A',
        A: 'U'
      };
      return rna[nucleo];
    }).join('')
};
