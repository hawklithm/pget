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
            url: "https://cdn-lfs-us-1.hf-mirror.com/repos/5d/66/5d66867ddd760feda1277de940d9bd0c5a4be4671860437942fae2d230446ccb/dadfd56d766715c61d2ef780a525ab43b8e6da4de6865bda3d95fdef5e134055?response-content-disposition=attachment%3B+filename*%3DUTF-8%27%27tokenizer.model%3B+filename%3D%22tokenizer.model%22%3B&Expires=1714119432&Policy=eyJTdGF0ZW1lbnQiOlt7IkNvbmRpdGlvbiI6eyJEYXRlTGVzc1RoYW4iOnsiQVdTOkVwb2NoVGltZSI6MTcxNDExOTQzMn19LCJSZXNvdXJjZSI6Imh0dHBzOi8vY2RuLWxmcy11cy0xLmh1Z2dpbmdmYWNlLmNvL3JlcG9zLzVkLzY2LzVkNjY4NjdkZGQ3NjBmZWRhMTI3N2RlOTQwZDliZDBjNWE0YmU0NjcxODYwNDM3OTQyZmFlMmQyMzA0NDZjY2IvZGFkZmQ1NmQ3NjY3MTVjNjFkMmVmNzgwYTUyNWFiNDNiOGU2ZGE0ZGU2ODY1YmRhM2Q5NWZkZWY1ZTEzNDA1NT9yZXNwb25zZS1jb250ZW50LWRpc3Bvc2l0aW9uPSoifV19&Signature=Z0WM0pwDlnMJ-qJPgrSx0bxczXhpiCHABe6FXt2rK6BZ007Wtt6bK7stwNyCJzwWnVjAPzXXaTfjXLKG6E4ZseP3h636To3QdsfWk3-krbNeV-V6cLl-e5vJoZuuuxgTayJrroE3WNu8FbVh%7EiuN88eEbUmm5IOlamaA5%7EzxGjucSKKrX07vIQjbVaIhdxweS4v3oED7WgN5okIctnvIZbcg%7E3XwFjChHiZsm6HFeGl67rVprTiWWLe7DFGgyh86wk7vGlmHVE4rups3zO0-0ObT2UMPcqKgK1tFtixzjbESSt4PtIWTtBTlhSHUG2ErmqYfOazz1d9O3nHvsaEGWQ__&Key-Pair-Id=KCD77M1F0VK2B".to_string(),
            memory: 256,
            filename: PathBuf::from("test.txt"),
            ..Default::default() 
        };

        download.get().unwrap();
    }

    #[test]
    fn test2(){
        download( "https://cdn-lfs-us-1.hf-mirror.com/repos/5d/66/5d66867ddd760feda1277de940d9bd0c5a4be4671860437942fae2d230446ccb/48a99d78aaea1949ad85dce7847fa7d033173a22a58add4b2cd732410e7e15cf?response-content-disposition=attachment%3B+filename*%3DUTF-8%27%27model-00022-of-00059.safetensors%3B+filename%3D%22model-00022-of-00059.safetensors%22%3B&Expires=1714118392&Policy=eyJTdGF0ZW1lbnQiOlt7IkNvbmRpdGlvbiI6eyJEYXRlTGVzc1RoYW4iOnsiQVdTOkVwb2NoVGltZSI6MTcxNDExODM5Mn19LCJSZXNvdXJjZSI6Imh0dHBzOi8vY2RuLWxmcy11cy0xLmh1Z2dpbmdmYWNlLmNvL3JlcG9zLzVkLzY2LzVkNjY4NjdkZGQ3NjBmZWRhMTI3N2RlOTQwZDliZDBjNWE0YmU0NjcxODYwNDM3OTQyZmFlMmQyMzA0NDZjY2IvNDhhOTlkNzhhYWVhMTk0OWFkODVkY2U3ODQ3ZmE3ZDAzMzE3M2EyMmE1OGFkZDRiMmNkNzMyNDEwZTdlMTVjZj9yZXNwb25zZS1jb250ZW50LWRpc3Bvc2l0aW9uPSoifV19&Signature=h5NUSQeBYigJpqSXdcqcnXfwKClGSvXuCaPsq%7E5XIfMuIkoO90I2Piz3hEwtRwWnZV7PsC9IWppPqEDzl3DaJxjqgXc3j%7ENwqTYnLUNiISpJk3cG8eKTqJ2xpy-gje%7Eakyq7pXgJAiAFFDkXgqefZQIKIGE5SoARucIfgEL%7E5mUER3xxqvqLdCtShpyIZ1APhXp3qTgue1DkMmErScyqCnjnEVAZs5%7EmePl1XUcNfJnWix1PtB5-bn4gcZ0YSxa%7ElAPNPzP-MOryMz1Sfol0aBBzcmMjedvUzmaD7MuJ-PwhskQSoajIBcuDoi6ikN00NcS5Du1U8Onteea%7E0wdpGQ__&Key-Pair-Id=KCD77M1F0VK2B".to_string(),
        num_cpus::get(),
            PathBuf::from("test.deb"),
        );

    }

    #[test]
    fn test3(){
        download( "https://cdn-lfs-us-1.hf-mirror.com/repos/5d/66/5d66867ddd760feda1277de940d9bd0c5a4be4671860437942fae2d230446ccb/e36fa9e3f053db7a27e06f5b2804a2aa650a026a66e3f7db9efd35c5103cd9ae?response-content-disposition=attachment%3B+filename*%3DUTF-8%27%27model-00059-of-00059.safetensors%3B+filename%3D%22model-00059-of-00059.safetensors%22%3B&Expires=1714124366&Policy=eyJTdGF0ZW1lbnQiOlt7IkNvbmRpdGlvbiI6eyJEYXRlTGVzc1RoYW4iOnsiQVdTOkVwb2NoVGltZSI6MTcxNDEyNDM2Nn19LCJSZXNvdXJjZSI6Imh0dHBzOi8vY2RuLWxmcy11cy0xLmh1Z2dpbmdmYWNlLmNvL3JlcG9zLzVkLzY2LzVkNjY4NjdkZGQ3NjBmZWRhMTI3N2RlOTQwZDliZDBjNWE0YmU0NjcxODYwNDM3OTQyZmFlMmQyMzA0NDZjY2IvZTM2ZmE5ZTNmMDUzZGI3YTI3ZTA2ZjViMjgwNGEyYWE2NTBhMDI2YTY2ZTNmN2RiOWVmZDM1YzUxMDNjZDlhZT9yZXNwb25zZS1jb250ZW50LWRpc3Bvc2l0aW9uPSoifV19&Signature=Kn%7EZZLKdwvndA%7E9-koQvcs7-Ja9NuVNuWTzyCHEiIFC-nHv-8F489ETgrwOKuH2w6rHKoMkCGYNyjX8d28vcYAWARyimlkSXyn7CDHYAFLuuSLVUvvgu1T2eIitpHD10rx21gAb05VSz3hDqe-1KsgoBZH7OUwIgXiv3UgJelUfpuoRJk5ZeadTCPiN7-YErAWKXjupvM-0yZbpOjD%7E4oKNzcc2imMJBpvWGMn5crJCW9E1L%7ElCUrPcu0LcEOi5JfjuNpetDbffPhJSUgt0Yqrz0L0tKhagvUed-YJTYpkrYS0MXsK1UsIVM60Jp4zYMGPDY3je7GLlJtmj%7EjDsmXA__&Key-Pair-Id=KCD77M1F0VK2B".to_string(),
        4,
            PathBuf::from("test.safetensors"),
        );

    }


}