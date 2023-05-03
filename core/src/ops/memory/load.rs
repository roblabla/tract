use crate::internal::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Load {
    pub id: String,
}

impl Load {
    pub fn new(id: &str) -> Load {
        Load { id: id.to_string() }
    }
}

impl Op for Load {
    fn name(&self) -> Cow<str> {
        "Load".into()
    }

    fn info(&self) -> TractResult<Vec<String>> {
        Ok(vec![format!("id: {:?}", self.id)])
    }

    impl_op_same_as!();
    op_as_typed_op!();
}

impl EvalOp for Load {
    fn is_stateless(&self) -> bool {
        false
    }

    fn state(
        &self,
        _session: &mut SessionState,
        _node_id: usize,
    ) -> TractResult<Option<Box<dyn OpState>>> {
        Ok(Some(Box::new(LoadState {})))
    }
}

impl TypedOp for Load {
    as_op!();

    fn output_facts(&self, inputs: &[&TypedFact]) -> TractResult<TVec<TypedFact>> {
        ensure!(inputs.len() == 1, "Expected one input (default value) for Load op");
        let input_facts = inputs.to_vec();
        Ok(input_facts.into_iter().cloned().collect::<TVec<_>>())
    }
}

#[derive(Debug, Clone)]
pub struct LoadState;

impl OpState for LoadState {
    fn eval(
        &mut self,
        session: &mut SessionState,
        op: &dyn Op,
        mut inputs: TVec<TValue>,
    ) -> TractResult<TVec<TValue>> {
        let input = args_1!(inputs);
        let load_op = op.downcast_ref::<Load>().ok_or(anyhow!("Expected Load op"))?;
        let tensor = session
            .tensors
            .get(&load_op.id)
            .map_or_else(
                || -> TractResult<TVec<TValue>> { Ok(tvec!(input.clone())) },
                |it| {
                    // Checks
                    ensure!(
                        it.datum_type() == input.datum_type(),
                        anyhow!(
                            "Expected datum {:?}, found {:?}",
                            input.datum_type(),
                            it.datum_type()
                        )
                    );
                    ensure!(
                        it.shape() == input.shape(),
                        anyhow!("Expected shape {:?}, found {:?}", input.shape(), it.shape())
                    );
                    Ok(tvec!(it.clone().into_tvalue()))
                },
            )
            .with_context(|| anyhow!("While loading tensor from session"))?;

        Ok(tensor)
    }
}

trivial_op_state_freeeze!(LoadState);