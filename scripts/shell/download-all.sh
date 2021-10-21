selfpath=$(cd "$(dirname "$0")"; pwd) 

rust_proj_path=$(cd ""${selfpath}"/../.."; pwd)
echo "detected project path : $rust_proj_path"

downloads_path=$(cd ""${rust_proj_path}"/../downloads"; pwd)
echo "downloads path : ${downloads_path}"


if [ ! -d ${downloads_path} ];
then 
    mkdir ${downloads_path}
    echo "mkdir ${downloads_path}"
fi

if [ ! -f  ${selfpath}"/../config" ];
then 
    echo "missing  ${selfpath}/config"
    exit 1
fi
cat ${selfpath}"/../config" | sed '/#/'d | sed -e '/^$/d' | while read cate_name
do
    echo "download for category ${cate_name}"
    cate_path="${downloads_path}/${cate_name}"
    if [ ! -d ${cate_path} ];
    then 
        mkdir ${cate_path}
        echo "mkdir ${cate_path}"
    fi

    read proj_cnt
    if [ proj_cnt != "0" ];
    then
        for i in `seq 1 ${proj_cnt}` ;
        do
            read url
            echo $url
            proj_name=`echo ${url} | sed 's/.git$//' |  awk -F'/' '{print $NF}'`

            if [ -d "${cate_path}/${proj_name}" ];
            then
                echo "skip download project : ${cate_name} / ${proj_name}"
            else
                echo "download for project : ${cate_name} / ${proj_name}"
                pushd "${cate_path}" > /dev/null
                git clone ${url}
                popd
            fi

        done
    fi
done