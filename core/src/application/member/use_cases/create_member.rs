use crate::{
    application::member::{
        ports::{
            create_member_input_port::{CreateMemberInputData, CreateMemberInputPort},
            create_member_output_port::{CreateMemberOutputData, CreateMemberOutputPort},
        },
        repositories::member_repository::MemberRepository,
    },
    domain::member::Member,
};

pub struct CreateMemberUseCase<R, OP>
where
    R: MemberRepository,
    OP: CreateMemberOutputPort,
{
    member_repo: R,
    output_port: OP,
}

impl<R, OP> CreateMemberUseCase<R, OP>
where
    R: MemberRepository,
    OP: CreateMemberOutputPort,
{
    pub fn new(member_repo: R, output_port: OP) -> Self {
        Self {
            member_repo,
            output_port,
        }
    }
}

#[async_trait::async_trait]
impl<R, OP> CreateMemberInputPort for CreateMemberUseCase<R, OP>
where
    R: MemberRepository,
    OP: CreateMemberOutputPort,
{
    async fn create(&self, import_data: CreateMemberInputData) {
        let new_id = self.member_repo.create_member(import_data.into()).await;

        // let's calling presentation method.
        let output_data = CreateMemberOutputData { new_id };
        self.output_port.present(output_data);
    }
}
