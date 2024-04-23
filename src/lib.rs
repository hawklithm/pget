use std::path::PathBuf;

mod download;

pub fn download<P:AsRef<str>>(url:P,thread:usize,output_file:PathBuf){
    let download = download::Download { 
        threads: thread,
        url: url.as_ref().to_owned(),
        memory: 256,
        filename: output_file,
        ..Default::default() 
    };

    download.get().unwrap();
}



#[cfg(test)]
mod test{
    use std::path::PathBuf;

    use crate::download;

    #[test]
    fn test(){
        let download = download::Download { 
            threads: 4,
            url: "https://hf-mirror.com/mistral-community/Mixtral-8x22B-v0.1/raw/main/model-00012-of-00059.safetensors".to_string(),
            memory: 256,
            filename: PathBuf::from(".\\test.bin"),
            ..Default::default() 
        };

        download.get().unwrap();
    }

    #[test]
    fn test2(){
        download( "https://cdn-lfs-us-1.hf-mirror.com/repos/44/39/443946633c5556133bdc3efcc7a98d9c7a2ae9309f90d6f445538267198d9dbf/875c9a5523173e0d2ae2ed7abde3a95b021cac1cffecf5d400d505aadc061032?response-content-disposition=attachment%3B+filename*%3DUTF-8%27%27model-00012-of-00059.safetensors%3B+filename%3D%22model-00012-of-00059.safetensors%22%3B&Expires=1714095953&Policy=eyJTdGF0ZW1lbnQiOlt7IkNvbmRpdGlvbiI6eyJEYXRlTGVzc1RoYW4iOnsiQVdTOkVwb2NoVGltZSI6MTcxNDA5NTk1M319LCJSZXNvdXJjZSI6Imh0dHBzOi8vY2RuLWxmcy11cy0xLmh1Z2dpbmdmYWNlLmNvL3JlcG9zLzQ0LzM5LzQ0Mzk0NjYzM2M1NTU2MTMzYmRjM2VmY2M3YTk4ZDljN2EyYWU5MzA5ZjkwZDZmNDQ1NTM4MjY3MTk4ZDlkYmYvODc1YzlhNTUyMzE3M2UwZDJhZTJlZDdhYmRlM2E5NWIwMjFjYWMxY2ZmZWNmNWQ0MDBkNTA1YWFkYzA2MTAzMj9yZXNwb25zZS1jb250ZW50LWRpc3Bvc2l0aW9uPSoifV19&Signature=i3628lY8vwf6YQtv8RBSDMnUpGnjqqS-KtvQ6IM0a571ocfhImsHMc%7EyMbo4VECUofLDlt7utte02QyMpy8dXmpoehe8-c-R3Ct1ZHeosaYOj7ilIKg8H7bv7vexGhe4K-GYepDL74FESZ9AWdrLJtWlXjfJIlccKqSSGuMkJjDPK0qnyzbHENORiiuhKm8boWDuq4Mk5QBSahkOmWDUtCYVROEmulGRDCPKLMSHOwkhQb5MhPq%7EHcLihJwigN1TA%7EX6WDeNuC6ASd3hDAL7MeCi-9-tm7d%7EONGww%7E1RP-R7TLCq2ciEqbXdU8suUGQwCZUzSTz7XyrSSXM2XJJHng__&Key-Pair-Id=KCD77M1F0VK2B".to_string(),
        num_cpus::get(),
            PathBuf::from(".\\test.bin"),
        );

    }


}