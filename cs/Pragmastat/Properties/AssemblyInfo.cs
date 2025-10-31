using System.Runtime.CompilerServices;
using Pragmastat.Properties;

[assembly: CLSCompliant(true)]

[assembly: InternalsVisibleTo("Pragmastat,PublicKey=" + PragmastatInfo.PublicKey)]
[assembly: InternalsVisibleTo("Pragmastat.Distributions,PublicKey=" + PragmastatInfo.PublicKey)]
[assembly: InternalsVisibleTo("Pragmastat.Estimators,PublicKey=" + PragmastatInfo.PublicKey)]
[assembly: InternalsVisibleTo("Pragmastat.Extended,PublicKey=" + PragmastatInfo.PublicKey)]

[assembly: InternalsVisibleTo("Pragmastat.ReferenceTests.Generator,PublicKey=" + PragmastatInfo.PublicKey)]
[assembly: InternalsVisibleTo("Pragmastat.ReferenceTests,PublicKey=" + PragmastatInfo.PublicKey)]
[assembly: InternalsVisibleTo("Pragmastat.UnitTests,PublicKey=" + PragmastatInfo.PublicKey)]
[assembly: InternalsVisibleTo("Pragmastat.Simulations,PublicKey=" + PragmastatInfo.PublicKey)]
