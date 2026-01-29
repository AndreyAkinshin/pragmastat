using System.Runtime.CompilerServices;
using Pragmastat.Properties;

[assembly: CLSCompliant(true)]

[assembly: InternalsVisibleTo("Pragmastat,PublicKey=" + PragmastatInfo.PublicKey)]
[assembly: InternalsVisibleTo("Pragmastat.Extended,PublicKey=" + PragmastatInfo.PublicKey)]

[assembly: InternalsVisibleTo("Pragmastat.TestGenerator,PublicKey=" + PragmastatInfo.PublicKey)]
[assembly: InternalsVisibleTo("Pragmastat.Tests,PublicKey=" + PragmastatInfo.PublicKey)]
[assembly: InternalsVisibleTo("Pragmastat.Simulations,PublicKey=" + PragmastatInfo.PublicKey)]
