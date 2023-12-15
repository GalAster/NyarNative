use super::*;

impl HIR for ClassDeclaration {
    fn send_module(self, ctx: &mut ModuleResolver) -> Result<Self::Output> {
        let symbol = self.name.as_namespace_symbol(&ctx.namespace);
        let mut class = ValkyrieStructure { symbol, fields: Default::default(), methods: Default::default() };

        for x in self.terms {
            match x {
                ClassTerm::Macro(_) => {}
                ClassTerm::Field(f) => {
                    let mut field = FieldDefinition::new(&f.name);
                    field.typing = f.typing;

                    class.add_field(field).ok();
                }
                ClassTerm::Method(_) => {}
                ClassTerm::Domain(_) => {}
            }
        }

        match ctx.items.entry(class.name()) {
            Entry::Occupied(e) => {
                todo!()
            }
            Entry::Vacant(e) => {
                e.insert(ModuleItem::Structure(class));
            }
        }
        Ok(())
    }
}
