use couchbase::Cluster;
use couchbase::subdoc::LookupInSpec;
use futures::Future;

fn main() {
    let mut cluster = Cluster::connect("couchbase://127.0.0.1", "Administrator", "password")
        .expect("Could not create Cluster reference!");
    let bucket = cluster
        .bucket("travel-sample")
        .expect("Could not open bucket");
    let collection = bucket.default_collection();

    // TODO: figure out why only fulldoc get works.. some issue with the paths?

    // Fetch only a partial list of fields
    let partial_result = collection.lookup_in("airport_1285", vec![LookupInSpec::count("geo")], None).wait();
    println!("Partial Result: {:?}", partial_result);

    // Fetch the full document (might be needed in combination with xattrs or macros)
    let full_result = collection.lookup_in("airline_10123", vec![LookupInSpec::get_full_document()], None).wait();
    println!("Full Result: {:?}", full_result);
}