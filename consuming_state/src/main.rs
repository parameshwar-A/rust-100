fn main() {

    struct IntakeVehicle{
        reg_no: String,
        mob_no: String,
        kms_driven: u32,
        service_no: u32
    }

    struct InProgressVehicle{
        reg_no: String,
        mob_no: String,
        kms_driven: u32,
        service_no: u32,
        service_id: String,
        parts_id: [i32;5]
    }

    struct CompletedVehicle{
        reg_no: String,
        mob_no: String,
        kms_driven: u32,
        service_id: String,
        cost: u32
    }

    let veh1 = IntakeVehicle{
        reg_no: String::from("TN21BV1234"),
        mob_no: String::from("8888888888"),
        kms_driven: 5400,
        service_no: 3
    };
    println!("Intake Status");
    println!("Registration no: {} \nMobile no: {} \nKms Reading: {} \nService no: {}", veh1.reg_no, veh1.mob_no, veh1.kms_driven, veh1.service_no);
    println!("");

    impl IntakeVehicle{
        fn transition(self) -> InProgressVehicle{
            let val = InProgressVehicle{
                service_id: String::from("ID1234"),
                parts_id:[1,2,3,4,5],
            reg_no: self.reg_no,
            mob_no: self.mob_no,
            kms_driven: self.kms_driven,
            service_no: self.service_no
            };
            return val
        }
    }
    impl InProgressVehicle{
        fn transition(self) -> CompletedVehicle{
            let val = CompletedVehicle{
                    reg_no: self.reg_no,
                    mob_no: self.mob_no,
                    kms_driven: self.kms_driven,
                    service_id: self.service_id,
                    cost: 5000 
            };
            val
        }
    }
    let veh1 = veh1.transition();
    println!("Inprogress Status");
    println!("Registration no: {} \nMobile no: {} \nKms Reading: {} \nService no: {} \nService Id: {} \nParts: {}", veh1.reg_no, veh1.mob_no, veh1.kms_driven, veh1.service_no, veh1.service_id, veh1.parts_id.len());
    println!("");
    let veh1 = veh1.transition();
    println!("CompletedVehicle status");
    println!("Registration no: {} \nMobile no: {} \nKms Reading: {} \nService Id: {} \ncost: {}", veh1.reg_no, veh1.mob_no, veh1.kms_driven, veh1.service_id, veh1.cost);
    println!("");
}
